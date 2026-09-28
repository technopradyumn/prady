export interface ParameterDoc {
  name: string;
  type: string;
  description: string;
  optional?: boolean;
  default?: string;
}

export interface ReturnTypeDoc {
  type: string;
  description: string;
}

export interface MethodDoc {
  name: string;
  signature: string;
  summary: string;
  parameters: ParameterDoc[];
  returnType: ReturnTypeDoc;
  timeComplexity: string;
  spaceComplexity: string;
  description: string;
  example: string;
  throws?: string[];
  notes?: string[];
}

export interface DocItem {
  id: string;
  title: string;
  category: string;
  subCategory?: string;
  breadcrumbs: string[];
  summary: string;
  badge?: string;
  constructorSyntax?: string;
  constructorParameters?: ParameterDoc[];
  timeComplexity?: string;
  spaceComplexity?: string;
  overview?: string;
  methods?: MethodDoc[];
  properties?: { name: string; type: string; description: string }[];
  example?: string;
  seeAlso?: { title: string; link: string }[];
  contentHtml?: string;
}

export interface SearchResult {
  id: string;
  methodName?: string;
  title: string;
  category: string;
  summary: string;
  url: string;
}
