use prady_interp::phases::*;

#[test]
fn test_phase3_codegen_and_llvm_ir() {
    let mut module = IrModule::new("test_prog");
    module.target_triple = "x86_64-pc-windows-msvc".to_string();

    let mut func = IrFunction::new("add", vec![("a".into(), IrType::I64), ("b".into(), IrType::I64)], IrType::I64);
    let mut bb = IrBasicBlock::new("entry");
    bb.push(IrInstr::Add {
        dest: "%res".into(),
        ty: IrType::I64,
        lhs: IrValue::Reg("a".into()),
        rhs: IrValue::Reg("b".into()),
    });
    bb.push(IrInstr::Ret {
        ty: IrType::I64,
        val: Some(IrValue::Reg("%res".into())),
    });
    func.blocks.push(bb);
    module.add_function(func);

    let mut codegen = CodeGenerator::new(module);
    let llvm_ir = codegen.emit();

    assert!(llvm_ir.contains("target triple = \"x86_64-pc-windows-msvc\""));
    assert!(llvm_ir.contains("define i64 @add(i64 %a, i64 %b)"));
    assert!(llvm_ir.contains("add i64 %a, %b"));
}

#[test]
fn test_phase4_patterns_and_exhaustiveness() {
    let mut def = EnumTypeDef::new("Option");
    def.add_variant("Some", AdtPayload::Tuple(vec!["Int".into()]));
    def.add_variant("None", AdtPayload::Unit);

    let instance = EnumInstance::new("Option", "Some", 0, vec!["42".into()]);
    let pattern = Pattern::Variant {
        enum_name: Some("Option".into()),
        variant_name: "Some".into(),
        sub_patterns: vec![Pattern::Variable("x".into())],
    };

    let matched = PatternEngine::try_match(&pattern, &instance);
    assert!(matched.is_some());
    assert_eq!(matched.unwrap().get("x").unwrap(), "42");

    let arms = vec![
        MatchArm { pattern: Pattern::Variant { enum_name: None, variant_name: "Some".into(), sub_patterns: vec![] }, guard: None, action_id: 1 },
        MatchArm { pattern: Pattern::Variant { enum_name: None, variant_name: "None".into(), sub_patterns: vec![] }, guard: None, action_id: 2 },
    ];
    let variants = vec!["Some".into(), "None".into()];
    assert!(PatternEngine::check_exhaustiveness(&arms, &variants).is_ok());
}

#[test]
fn test_phase5_dsa_specifications() {
    let specs = get_dsa_specifications();
    assert!(specs.len() >= 8);
    let vec_spec = specs.iter().find(|s| s.name == "Vector").unwrap();
    assert_eq!(vec_spec.operations[0].operation, "get(index)");
    assert_eq!(vec_spec.operations[0].time_average, BigO::O1);
}

#[test]
fn test_phase6_clean_architecture_policy() {
    let mut engine = ArchitecturePolicyEngine::new("CleanArch");
    engine.add_layer("domain");
    engine.add_layer("application");
    engine.add_layer("infrastructure");

    engine.allow_flow("application", "domain");
    engine.deny_flow("domain", "infrastructure");

    engine.map_file_to_layer("src/domain/user.pr", "domain");
    engine.map_file_to_layer("src/infra/db.pr", "infrastructure");

    // Domain importing infrastructure must fail!
    let result = engine.check_import("src/domain/user.pr", "src/infra/db.pr", 10);
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert_eq!(err.from_layer, "domain");
    assert_eq!(err.to_layer, "infrastructure");
}

#[test]
fn test_phase7_async_and_router() {
    let mut router = HttpRouter::new();
    router.get("/health", |_req| HttpResponse::ok("OK"));

    let req = HttpRequest::new(HttpMethod::Get, "/health");
    let resp = router.handle(&req);
    assert_eq!(resp.status, 200);
    assert_eq!(resp.text(), "OK");

    let mut runtime = AsyncRuntime::new();
    let mut count = 0;
    runtime.spawn("task1", move || {
        count += 1;
        if count >= 3 {
            TaskStatus::Completed
        } else {
            TaskStatus::Yielded
        }
    });
    runtime.run_until_complete();
}

#[test]
fn test_phase8_tooling_and_lint() {
    let unformatted = "fn test() {\nlet x = 1;\nreturn x;\n}";
    let formatter = CodeFormatter::new();
    let formatted = formatter.format(unformatted);
    assert!(formatted.contains("    let x = 1;"));

    let code_with_smell = "fn BadName() {}\n";
    let lints = Linter::lint_source(code_with_smell);
    assert!(!lints.is_empty());
    assert_eq!(lints[0].rule, "naming/snake-case");
}

#[test]
fn test_phase10_security_sandbox() {
    let sandbox = SecurityPolicy::strict_sandbox();
    assert!(sandbox.check_network_access("api.example.com").is_err());
    assert!(sandbox.check_filesystem_write("secrets.txt").is_err());

    let permissive = SecurityPolicy::permissive();
    assert!(permissive.check_network_access("api.example.com").is_ok());
    assert!(permissive.check_filesystem_write("test.txt").is_ok());
}

#[test]
fn test_phase11_stabilization_release() {
    assert!(ReleaseManager::validate_semver("1.0.0"));
    assert!(ReleaseManager::validate_semver("2.1.0-alpha"));
    assert!(!ReleaseManager::validate_semver("v1"));

    let tests = ConformanceSuite::get_standard_tests();
    assert!(tests.len() >= 4);
    assert_eq!(tests[0].id, "CONF-001");
}
