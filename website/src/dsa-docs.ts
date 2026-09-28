import { DocItem } from './types';

export const DSA_DOCS: DocItem[] = [
  {
    id: 'dsa-array',
    title: 'Array / List',
    category: 'Data Structures',
    subCategory: 'Sequential Collections',
    breadcrumbs: ['References', 'Data Structures', 'Array'],
    summary: 'A dynamically-sized sequential collection with O(1) indexed access and comprehensive higher-order functional methods.',
    badge: 'Standard Library',
    constructorSyntax: 'let arr = [element0, element1, ...];\nlet list = Array();',
    constructorParameters: [],
    timeComplexity: 'Access: O(1) | Append: O(1) amortized | Prepend: O(n)',
    spaceComplexity: 'O(n)',
    overview: 'The Prady Array is a contiguous, resizable array. It supports standard indexing (arr[i]), functional manipulation (map, filter, reduce), in-place modification (push, pop, shift, splice), and search operations.',
    methods: [
      {
        name: 'map',
        signature: 'array.map(callback: fn(item: T, index: Int) -> U) -> Array<U>',
        summary: 'Creates a new array populated with the results of calling the provided function on every element.',
        parameters: [
          { name: 'callback', type: 'fn(item: T, index: Int) -> U', description: 'Function that produces an element of the new Array, taking the item and index.' }
        ],
        returnType: { type: 'Array<U>', description: 'A new array with each element being the result of the callback function.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(n)',
        description: 'Calls a provided callback function once for each element in an array, in order, and constructs a new array from the results.',
        example: 'let nums = [1, 2, 3, 4];\nlet doubled = nums.map(fn(x: Int, i: Int) -> Int {\n    return x * 2;\n});\nprint(doubled); // [2, 4, 6, 8]'
      },
      {
        name: 'filter',
        signature: 'array.filter(predicate: fn(item: T, index: Int) -> Bool) -> Array<T>',
        summary: 'Creates a shallow copy of a portion of a given array, filtered down to just the elements that pass the predicate test.',
        parameters: [
          { name: 'predicate', type: 'fn(item: T, index: Int) -> Bool', description: 'Function to test each element. Returns true to keep, false otherwise.' }
        ],
        returnType: { type: 'Array<T>', description: 'A shallow copy of the given array containing elements that pass the test.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(n)',
        description: 'Iterates over elements of the array, returning a new array containing all elements for which predicate returns truthy.',
        example: 'let nums = [10, 15, 20, 25];\nlet evens = nums.filter(fn(x: Int, i: Int) -> Bool {\n    return x % 2 == 0;\n});\nprint(evens); // [10, 20]'
      },
      {
        name: 'reduce',
        signature: 'array.reduce(reducer: fn(accumulator: U, item: T, index: Int) -> U, initialValue?: U) -> U',
        summary: 'Executes a user-supplied reducer callback function on each element of the array, resulting in a single output value.',
        parameters: [
          { name: 'reducer', type: 'fn(acc: U, item: T, index: Int) -> U', description: 'A function that accepts the accumulated value and current element.' },
          { name: 'initialValue', type: 'U', description: 'A value to use as the first argument to the first call of the reducer.', optional: true }
        ],
        returnType: { type: 'U', description: 'The value that results from running the reducer callback function across the entire array.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(1)',
        description: 'Iterates through the elements in order, passing the accumulated result to the next iteration.',
        example: 'let nums = [1, 2, 3, 4, 5];\nlet total = nums.reduce(fn(acc: Int, x: Int, i: Int) -> Int {\n    return acc + x;\n}, 0);\nprint(total); // 15'
      },
      {
        name: 'forEach',
        signature: 'array.forEach(action: fn(item: T, index: Int) -> Null) -> Null',
        summary: 'Executes a provided function once for each array element.',
        parameters: [
          { name: 'action', type: 'fn(item: T, index: Int) -> Null', description: 'Function to execute for each element.' }
        ],
        returnType: { type: 'Null', description: 'Always returns null.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(1)',
        description: 'Calls the callback on each element in ascending index order. Does not mutate the array unless the callback does so.',
        example: 'let names = ["Alice", "Bob", "Charlie"];\nnames.forEach(fn(name: String, i: Int) {\n    print("Hello " + name);\n});'
      },
      {
        name: 'push',
        signature: 'array.push(...elements: T) -> Null',
        summary: 'Appends one or more elements to the end of an array.',
        parameters: [
          { name: '...elements', type: 'T', description: 'The element(s) to add to the end of the array.' }
        ],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(1) amortized',
        spaceComplexity: 'O(1)',
        description: 'Inserts elements at the end, reallocating underlying capacity if needed.',
        example: 'let arr = [1, 2];\narr.push(3);\nprint(arr); // [1, 2, 3]'
      },
      {
        name: 'pop',
        signature: 'array.pop() -> T | Null',
        summary: 'Removes the last element from an array and returns that element.',
        parameters: [],
        returnType: { type: 'T | Null', description: 'The removed element from the array; null if the array is empty.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Pops the top/end element from the array.',
        example: 'let arr = [10, 20, 30];\nlet last = arr.pop();\nprint(last); // 30'
      },
      {
        name: 'shift',
        signature: 'array.shift() -> T | Null',
        summary: 'Removes the first element from an array and returns that removed element.',
        parameters: [],
        returnType: { type: 'T | Null', description: 'The removed element from the array; null if empty.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(1)',
        description: 'Shifts all remaining elements down by one position.',
        example: 'let arr = [1, 2, 3];\nlet first = arr.shift();\nprint(first); // 1'
      },
      {
        name: 'unshift',
        signature: 'array.unshift(...elements: T) -> Int',
        summary: 'Inserts given elements to the beginning of an array and returns the new length.',
        parameters: [
          { name: '...elements', type: 'T', description: 'The elements to insert at front.' }
        ],
        returnType: { type: 'Int', description: 'The new length of the array.' },
        timeComplexity: 'O(n + k)',
        spaceComplexity: 'O(1)',
        description: 'Inserts elements at index 0, shifting existing elements to the right.',
        example: 'let arr = [3, 4];\narr.unshift(1, 2);\nprint(arr); // [1, 2, 3, 4]'
      },
      {
        name: 'find',
        signature: 'array.find(predicate: fn(item: T, index: Int) -> Bool) -> T | Null',
        summary: 'Returns the first element in the provided array that satisfies the provided testing function.',
        parameters: [
          { name: 'predicate', type: 'fn(item: T, index: Int) -> Bool', description: 'Function executed on each value.' }
        ],
        returnType: { type: 'T | Null', description: 'The first element that matches; null if not found.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(1)',
        description: 'Searches linearly from index 0 to end.',
        example: 'let users = [10, 20, 30, 40];\nlet found = users.find(fn(x: Int, i: Int) -> Bool { return x > 25; });\nprint(found); // 30'
      },
      {
        name: 'findIndex',
        signature: 'array.findIndex(predicate: fn(item: T, index: Int) -> Bool) -> Int',
        summary: 'Returns the index of the first element in an array that satisfies the provided testing function.',
        parameters: [
          { name: 'predicate', type: 'fn(item: T, index: Int) -> Bool', description: 'Function executed on each value.' }
        ],
        returnType: { type: 'Int', description: 'Index of element, or -1 if none found.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(1)',
        description: 'Returns the index of the first matching item or -1.',
        example: 'let idx = [5, 12, 8].findIndex(fn(x: Int, i: Int) -> Bool { return x > 10; });\nprint(idx); // 1'
      },
      {
        name: 'some',
        signature: 'array.some(predicate: fn(item: T, index: Int) -> Bool) -> Bool',
        summary: 'Tests whether at least one element in the array passes the test implemented by the provided function.',
        parameters: [
          { name: 'predicate', type: 'fn(item: T, index: Int) -> Bool', description: 'Predicate function.' }
        ],
        returnType: { type: 'Bool', description: 'True if predicate returns true for any element.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(1)',
        description: 'Short-circuits upon the first truthy result.',
        example: 'let hasBig = [1, 2, 9, 3].some(fn(x: Int, i: Int) -> Bool { return x > 8; });\nprint(hasBig); // true'
      },
      {
        name: 'every',
        signature: 'array.every(predicate: fn(item: T, index: Int) -> Bool) -> Bool',
        summary: 'Tests whether all elements in the array pass the test implemented by the provided function.',
        parameters: [
          { name: 'predicate', type: 'fn(item: T, index: Int) -> Bool', description: 'Predicate function.' }
        ],
        returnType: { type: 'Bool', description: 'True if all elements pass.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(1)',
        description: 'Short-circuits upon the first falsy result.',
        example: 'let allPos = [1, 2, 3].every(fn(x: Int, i: Int) -> Bool { return x > 0; });\nprint(allPos); // true'
      },
      {
        name: 'slice',
        signature: 'array.slice(start?: Int, end?: Int) -> Array<T>',
        summary: 'Returns a shallow copy of a portion of an array into a new array object selected from start to end (end not included).',
        parameters: [
          { name: 'start', type: 'Int', description: 'Zero-based index at which to start extraction.', optional: true },
          { name: 'end', type: 'Int', description: 'Zero-based index before which to end extraction.', optional: true }
        ],
        returnType: { type: 'Array<T>', description: 'A new array containing the extracted elements.' },
        timeComplexity: 'O(k)',
        spaceComplexity: 'O(k)',
        description: 'Negative indices count back from the end of the array.',
        example: 'let arr = [10, 20, 30, 40, 50];\nprint(arr.slice(1, 4)); // [20, 30, 40]'
      },
      {
        name: 'splice',
        signature: 'array.splice(start: Int, deleteCount: Int, ...items: T) -> Array<T>',
        summary: 'Changes the contents of an array by removing or replacing existing elements and/or adding new elements in place.',
        parameters: [
          { name: 'start', type: 'Int', description: 'The index at which to start changing the array.' },
          { name: 'deleteCount', type: 'Int', description: 'An integer indicating the number of elements to remove.' },
          { name: '...items', type: 'T', description: 'The elements to add to the array, beginning from start.', optional: true }
        ],
        returnType: { type: 'Array<T>', description: 'An array containing the deleted elements.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(k)',
        description: 'Modifies the array in place and returns the removed items.',
        example: 'let arr = [1, 2, 5];\narr.splice(2, 0, 3, 4);\nprint(arr); // [1, 2, 3, 4, 5]'
      },
      {
        name: 'join',
        signature: 'array.join(separator?: String) -> String',
        summary: 'Creates and returns a new string by concatenating all of the elements in an array, separated by commas or a specified separator string.',
        parameters: [
          { name: 'separator', type: 'String', description: 'String used to separate elements. Defaults to ",".', optional: true }
        ],
        returnType: { type: 'String', description: 'A string with all array elements joined.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(n)',
        description: 'Converts each element to string and joins them.',
        example: 'let arr = ["fire", "air", "water"];\nprint(arr.join(" - ")); // "fire - air - water"'
      },
      {
        name: 'reverse',
        signature: 'array.reverse() -> Array<T>',
        summary: 'Reverses an array in place and returns the reference to the same array.',
        parameters: [],
        returnType: { type: 'Array<T>', description: 'The reversed array.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(1)',
        description: 'Transposes the elements of the calling array in place.',
        example: 'let arr = [1, 2, 3];\narr.reverse();\nprint(arr); // [3, 2, 1]'
      },
      {
        name: 'sort',
        signature: 'array.sort() -> Array<T>',
        summary: 'Sorts the elements of an array in place and returns the reference to the same array.',
        parameters: [],
        returnType: { type: 'Array<T>', description: 'The sorted array.' },
        timeComplexity: 'O(n log n)',
        spaceComplexity: 'O(log n)',
        description: 'Sorts integers, floats, and strings in natural ascending order.',
        example: 'let arr = [40, 10, 30, 20];\narr.sort();\nprint(arr); // [10, 20, 30, 40]'
      },
      {
        name: 'includes',
        signature: 'array.includes(value: T) -> Bool',
        summary: 'Determines whether an array includes a certain value among its entries, returning true or false as appropriate.',
        parameters: [
          { name: 'value', type: 'T', description: 'The value to search for.' }
        ],
        returnType: { type: 'Bool', description: 'True if found, false otherwise.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(1)',
        description: 'Compares using structural equality.',
        example: 'print([1, 2, 3].includes(2)); // true'
      },
      {
        name: 'indexOf',
        signature: 'array.indexOf(value: T) -> Int',
        summary: 'Returns the first index at which a given element can be found in the array, or -1 if it is not present.',
        parameters: [
          { name: 'value', type: 'T', description: 'Element to locate in the array.' }
        ],
        returnType: { type: 'Int', description: 'The first index of the element; -1 if not found.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(1)',
        description: 'Finds index of element.',
        example: 'print(["a", "b", "c"].indexOf("b")); // 1'
      },
      {
        name: 'len',
        signature: 'array.len() -> Int',
        summary: 'Returns the number of elements in the array.',
        parameters: [],
        returnType: { type: 'Int', description: 'Total item count.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Also available via array.length() or len(array).',
        example: 'print([1, 2, 3].len()); // 3'
      },
      {
        name: 'at',
        signature: 'array.at(index: Int) -> T | Null',
        summary: 'Takes an integer value and returns the item at that index, allowing for positive and negative integers.',
        parameters: [
          { name: 'index', type: 'Int', description: 'Zero-based index. Negative index counts from end.' }
        ],
        returnType: { type: 'T | Null', description: 'Element at index or null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'arr.at(-1) returns the last element.',
        example: 'let arr = [10, 20, 30];\nprint(arr.at(-1)); // 30'
      }
    ]
  },
  {
    id: 'dsa-map',
    title: 'Map / HashMap',
    category: 'Data Structures',
    subCategory: 'Keyed Collections',
    breadcrumbs: ['References', 'Data Structures', 'Map'],
    summary: 'A hash-table backed key-value dictionary with average O(1) insertion, retrieval, and deletion.',
    badge: 'Standard Library',
    constructorSyntax: 'let map = Map();\nlet hmap = HashMap();',
    constructorParameters: [],
    timeComplexity: 'Get: O(1) avg | Set: O(1) avg | Delete: O(1) avg',
    spaceComplexity: 'O(n)',
    overview: 'The Map object holds key-value pairs and remembers the insertion order of the keys. Any value (both objects and primitive values) may be used as either a key or a value.',
    methods: [
      {
        name: 'set',
        signature: 'map.set(key: K, value: V) -> Null',
        summary: 'Sets the value for the key in the Map object.',
        parameters: [
          { name: 'key', type: 'K', description: 'The key of the element to add to the Map.' },
          { name: 'value', type: 'V', description: 'The value of the element to add to the Map.' }
        ],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(1) average',
        spaceComplexity: 'O(1)',
        description: 'Updates value if key already exists, otherwise inserts new entry.',
        example: 'let m = Map();\nm.set("host", "localhost");\nm.set("port", 8080);'
      },
      {
        name: 'get',
        signature: 'map.get(key: K) -> V | Null',
        summary: 'Returns a specified element from a Map object.',
        parameters: [
          { name: 'key', type: 'K', description: 'The key of the element to return.' }
        ],
        returnType: { type: 'V | Null', description: 'The element associated with the key, or null if key cannot be found.' },
        timeComplexity: 'O(1) average',
        spaceComplexity: 'O(1)',
        description: 'Looks up the key in the hash table.',
        example: 'let port = m.get("port");\nprint(port); // 8080'
      },
      {
        name: 'has',
        signature: 'map.has(key: K) -> Bool',
        summary: 'Returns a boolean indicating whether an element with the specified key exists or not.',
        parameters: [
          { name: 'key', type: 'K', description: 'The key of the element to test for presence.' }
        ],
        returnType: { type: 'Bool', description: 'True if key exists, false otherwise.' },
        timeComplexity: 'O(1) average',
        spaceComplexity: 'O(1)',
        description: 'Checks membership.',
        example: 'print(m.has("host")); // true'
      },
      {
        name: 'delete',
        signature: 'map.delete(key: K) -> Bool',
        summary: 'Removes the specified element from a Map object by key.',
        parameters: [
          { name: 'key', type: 'K', description: 'The key of the element to remove.' }
        ],
        returnType: { type: 'Bool', description: 'True if an element existed and has been removed, or false if not found.' },
        timeComplexity: 'O(1) average',
        spaceComplexity: 'O(1)',
        description: 'Removes the key-value pair and decreases map size.',
        example: 'let removed = m.delete("port");\nprint(removed); // true'
      },
      {
        name: 'clear',
        signature: 'map.clear() -> Null',
        summary: 'Removes all elements from a Map object.',
        parameters: [],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(1)',
        description: 'Empties the map.',
        example: 'm.clear();\nprint(m.size()); // 0'
      },
      {
        name: 'size',
        signature: 'map.size() -> Int',
        summary: 'Returns the number of elements in a Map object.',
        parameters: [],
        returnType: { type: 'Int', description: 'Count of key-value pairs.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Returns the number of entries.',
        example: 'print(m.size());'
      },
      {
        name: 'keys',
        signature: 'map.keys() -> Array<K>',
        summary: 'Returns an array of keys in insertion order.',
        parameters: [],
        returnType: { type: 'Array<K>', description: 'List of keys.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(n)',
        description: 'Extracts all keys.',
        example: 'print(m.keys()); // ["host", "port"]'
      },
      {
        name: 'values',
        signature: 'map.values() -> Array<V>',
        summary: 'Returns an array of values in insertion order.',
        parameters: [],
        returnType: { type: 'Array<V>', description: 'List of values.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(n)',
        description: 'Extracts all values.',
        example: 'print(m.values()); // ["localhost", 8080]'
      },
      {
        name: 'entries',
        signature: 'map.entries() -> Array<Array<Any>>',
        summary: 'Returns an array of [key, value] pairs.',
        parameters: [],
        returnType: { type: 'Array<Array<Any>>', description: 'Array of [key, value] pairs.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(n)',
        description: 'Extracts key-value pairs.',
        example: 'for (let entry of m.entries()) {\n    print(entry[0] + ": " + entry[1]);\n}'
      },
      {
        name: 'forEach',
        signature: 'map.forEach(callback: fn(entry: Array<Any>, index: Int)) -> Null',
        summary: 'Executes a provided function once per each key/value pair in the Map object.',
        parameters: [
          { name: 'callback', type: 'fn(entry: Array<Any>, index: Int)', description: 'Function executed for each entry.' }
        ],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(1)',
        description: 'Iterates through each [key, value] entry.',
        example: 'm.forEach(fn(entry: Array<Any>, i: Int) {\n    print(entry[0] + " => " + entry[1]);\n});'
      }
    ]
  },
  {
    id: 'dsa-set',
    title: 'Set / HashSet',
    category: 'Data Structures',
    subCategory: 'Keyed Collections',
    breadcrumbs: ['References', 'Data Structures', 'Set'],
    summary: 'A collection of unique values with O(1) membership lookup, set operations (union, intersection, difference).',
    badge: 'Standard Library',
    constructorSyntax: 'let set = Set();\nlet setWithItems = Set([1, 2, 3]);',
    constructorParameters: [
      { name: 'initialItems', type: 'Array<T>', description: 'Optional initial elements to seed into the set.', optional: true }
    ],
    timeComplexity: 'Add: O(1) avg | Has: O(1) avg | Delete: O(1) avg',
    spaceComplexity: 'O(n)',
    overview: 'The Set object lets you store unique values of any type. Duplicate entries are automatically ignored.',
    methods: [
      {
        name: 'add',
        signature: 'set.add(value: T) -> Null',
        summary: 'Appends a new element with a specified value to the end of the Set object.',
        parameters: [
          { name: 'value', type: 'T', description: 'The value of the element to add.' }
        ],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(1) average',
        spaceComplexity: 'O(1)',
        description: 'If value already exists in the set, the operation is a no-op.',
        example: 'let s = Set();\ns.add(10);\ns.add(20);\ns.add(10); // ignored'
      },
      {
        name: 'has',
        signature: 'set.has(value: T) -> Bool',
        summary: 'Returns a boolean asserting whether an element is present with the given value in the Set object or not.',
        parameters: [
          { name: 'value', type: 'T', description: 'The value to test.' }
        ],
        returnType: { type: 'Bool', description: 'True if present, false otherwise.' },
        timeComplexity: 'O(1) average',
        spaceComplexity: 'O(1)',
        description: 'Instant O(1) hash check.',
        example: 'print(s.has(20)); // true'
      },
      {
        name: 'delete',
        signature: 'set.delete(value: T) -> Bool',
        summary: 'Removes a specified value from the Set.',
        parameters: [
          { name: 'value', type: 'T', description: 'The value to delete.' }
        ],
        returnType: { type: 'Bool', description: 'True if value existed and was removed.' },
        timeComplexity: 'O(1) average',
        spaceComplexity: 'O(1)',
        description: 'Removes the item from the set.',
        example: 's.delete(10);'
      },
      {
        name: 'size',
        signature: 'set.size() -> Int',
        summary: 'Returns the number of (unique) elements in the Set.',
        parameters: [],
        returnType: { type: 'Int', description: 'Count of unique items.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Count of items.',
        example: 'print(s.size()); // 1'
      },
      {
        name: 'union',
        signature: 'set.union(other: Set<T>) -> Set<T>',
        summary: 'Returns a new Set containing elements from both this set and the other set.',
        parameters: [
          { name: 'other', type: 'Set<T>', description: 'Another set.' }
        ],
        returnType: { type: 'Set<T>', description: 'New set with union of elements.' },
        timeComplexity: 'O(n + m)',
        spaceComplexity: 'O(n + m)',
        description: 'Mathematical set union A ∪ B.',
        example: 'let a = Set([1, 2]);\nlet b = Set([2, 3]);\nlet u = a.union(b);\nprint(u.toArray()); // [1, 2, 3]'
      },
      {
        name: 'intersection',
        signature: 'set.intersection(other: Set<T>) -> Set<T>',
        summary: 'Returns a new Set containing elements present in both sets.',
        parameters: [
          { name: 'other', type: 'Set<T>', description: 'Another set.' }
        ],
        returnType: { type: 'Set<T>', description: 'New set with common elements.' },
        timeComplexity: 'O(min(n, m))',
        spaceComplexity: 'O(min(n, m))',
        description: 'Mathematical set intersection A ∩ B.',
        example: 'let common = a.intersection(b);\nprint(common.toArray()); // [2]'
      },
      {
        name: 'difference',
        signature: 'set.difference(other: Set<T>) -> Set<T>',
        summary: 'Returns a new Set containing elements present in this set but not the other.',
        parameters: [
          { name: 'other', type: 'Set<T>', description: 'Another set.' }
        ],
        returnType: { type: 'Set<T>', description: 'New set with relative difference.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(n)',
        description: 'Mathematical set difference A \\ B.',
        example: 'let diff = a.difference(b);\nprint(diff.toArray()); // [1]'
      },
      {
        name: 'toArray',
        signature: 'set.toArray() -> Array<T>',
        summary: 'Converts the set elements into a standard Array.',
        parameters: [],
        returnType: { type: 'Array<T>', description: 'Array of elements.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(n)',
        description: 'Exports set items.',
        example: 'let list = s.toArray();'
      }
    ]
  },
  {
    id: 'dsa-stack',
    title: 'Stack',
    category: 'Data Structures',
    subCategory: 'Sequential Collections',
    breadcrumbs: ['References', 'Data Structures', 'Stack'],
    summary: 'A Last-In, First-Out (LIFO) data container supporting O(1) push, pop, and peek operations.',
    badge: 'Standard Library',
    constructorSyntax: 'let stack = Stack();',
    constructorParameters: [],
    timeComplexity: 'Push: O(1) | Pop: O(1) | Peek: O(1)',
    spaceComplexity: 'O(n)',
    overview: 'Stack is ideal for expression evaluation, backtracking, undo/redo buffers, and depth-first searches.',
    methods: [
      {
        name: 'push',
        signature: 'stack.push(item: T) -> Null',
        summary: 'Pushes an item onto the top of the stack.',
        parameters: [{ name: 'item', type: 'T', description: 'Item to push.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Places item on top of the stack.',
        example: 'let s = Stack();\ns.push(10);\ns.push(20);'
      },
      {
        name: 'pop',
        signature: 'stack.pop() -> T | Null',
        summary: 'Removes and returns the item from the top of the stack.',
        parameters: [],
        returnType: { type: 'T | Null', description: 'Top item or null if empty.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Removes top element.',
        example: 'print(s.pop()); // 20'
      },
      {
        name: 'peek',
        signature: 'stack.peek() -> T | Null',
        summary: 'Returns the top item without removing it.',
        parameters: [],
        returnType: { type: 'T | Null', description: 'Top item or null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Inspects top element.',
        example: 'print(s.peek()); // 10'
      },
      {
        name: 'isEmpty',
        signature: 'stack.isEmpty() -> Bool',
        summary: 'Checks if stack has no elements.',
        parameters: [],
        returnType: { type: 'Bool', description: 'True if empty.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Checks emptiness.',
        example: 'if (!s.isEmpty()) { s.pop(); }'
      },
      {
        name: 'size',
        signature: 'stack.size() -> Int',
        summary: 'Returns the number of items on the stack.',
        parameters: [],
        returnType: { type: 'Int', description: 'Item count.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Returns size.',
        example: 'print(s.size());'
      }
    ]
  },
  {
    id: 'dsa-queue',
    title: 'Queue',
    category: 'Data Structures',
    subCategory: 'Sequential Collections',
    breadcrumbs: ['References', 'Data Structures', 'Queue'],
    summary: 'A First-In, First-Out (FIFO) data structure with O(1) enqueue and dequeue operations.',
    badge: 'Standard Library',
    constructorSyntax: 'let q = Queue();',
    constructorParameters: [],
    timeComplexity: 'Enqueue: O(1) | Dequeue: O(1) | Peek: O(1)',
    spaceComplexity: 'O(n)',
    overview: 'Queue is used in job scheduling, breadth-first search, rate limiting, and message buffers.',
    methods: [
      {
        name: 'enqueue',
        signature: 'queue.enqueue(item: T) -> Null',
        summary: 'Inserts an item at the back of the queue.',
        parameters: [{ name: 'item', type: 'T', description: 'Item to enqueue.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Appends to back of queue.',
        example: 'let q = Queue();\nq.enqueue("job1");\nq.enqueue("job2");'
      },
      {
        name: 'dequeue',
        signature: 'queue.dequeue() -> T | Null',
        summary: 'Removes and returns the item from the front of the queue.',
        parameters: [],
        returnType: { type: 'T | Null', description: 'Front item or null if empty.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Pulls the oldest item.',
        example: 'print(q.dequeue()); // "job1"'
      },
      {
        name: 'peek',
        signature: 'queue.peek() -> T | Null',
        summary: 'Returns front item without removing it.',
        parameters: [],
        returnType: { type: 'T | Null', description: 'Front item.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Inspects head.',
        example: 'print(q.peek()); // "job2"'
      },
      {
        name: 'isEmpty',
        signature: 'queue.isEmpty() -> Bool',
        summary: 'Checks if queue is empty.',
        parameters: [],
        returnType: { type: 'Bool', description: 'True if empty.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Checks emptiness.',
        example: 'print(q.isEmpty());'
      },
      {
        name: 'size',
        signature: 'queue.size() -> Int',
        summary: 'Returns number of items in queue.',
        parameters: [],
        returnType: { type: 'Int', description: 'Item count.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Returns size.',
        example: 'print(q.size());'
      }
    ]
  },
  {
    id: 'dsa-deque',
    title: 'Deque (Double-Ended Queue)',
    category: 'Data Structures',
    subCategory: 'Sequential Collections',
    breadcrumbs: ['References', 'Data Structures', 'Deque'],
    summary: 'A double-ended queue supporting O(1) push and pop from both ends.',
    badge: 'Standard Library',
    constructorSyntax: 'let dq = Deque();',
    constructorParameters: [],
    timeComplexity: 'Push/Pop Front/Back: O(1)',
    spaceComplexity: 'O(n)',
    overview: 'Deque allows efficient insertion and removal at both the beginning and the end.',
    methods: [
      {
        name: 'pushFront',
        signature: 'deque.pushFront(item: T) -> Null',
        summary: 'Inserts item at front.',
        parameters: [{ name: 'item', type: 'T', description: 'Item to insert.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Prepends to deque.',
        example: 'dq.pushFront(1);'
      },
      {
        name: 'pushBack',
        signature: 'deque.pushBack(item: T) -> Null',
        summary: 'Inserts item at back.',
        parameters: [{ name: 'item', type: 'T', description: 'Item to insert.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Appends to deque.',
        example: 'dq.pushBack(2);'
      },
      {
        name: 'popFront',
        signature: 'deque.popFront() -> T | Null',
        summary: 'Removes and returns front item.',
        parameters: [],
        returnType: { type: 'T | Null', description: 'Front item or null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Removes front element.',
        example: 'print(dq.popFront()); // 1'
      },
      {
        name: 'popBack',
        signature: 'deque.popBack() -> T | Null',
        summary: 'Removes and returns back item.',
        parameters: [],
        returnType: { type: 'T | Null', description: 'Back item or null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Removes back element.',
        example: 'print(dq.popBack()); // 2'
      }
    ]
  },
  {
    id: 'dsa-minheap',
    title: 'PriorityQueue / MinHeap',
    category: 'Data Structures',
    subCategory: 'Trees & Heaps',
    breadcrumbs: ['References', 'Data Structures', 'MinHeap'],
    summary: 'A binary min-heap implementing a priority queue with O(log n) insertion and extraction of minimum element.',
    badge: 'Standard Library',
    constructorSyntax: 'let heap = MinHeap();\nlet pq = PriorityQueue();',
    constructorParameters: [],
    timeComplexity: 'Insert: O(log n) | ExtractMin: O(log n) | Peek: O(1)',
    spaceComplexity: 'O(n)',
    overview: 'MinHeap maintains the invariant that parent nodes are less than or equal to their children. Perfect for Dijkstra algorithm, Huffman coding, and streaming median algorithms.',
    methods: [
      {
        name: 'insert',
        signature: 'heap.insert(item: T) -> Null',
        summary: 'Adds a new value into the heap and restores heap invariant.',
        parameters: [{ name: 'item', type: 'T', description: 'Value to insert.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(log n)',
        spaceComplexity: 'O(1)',
        description: 'Pushes to end and sifts up.',
        example: 'let h = MinHeap();\nh.insert(30);\nh.insert(10);\nh.insert(20);'
      },
      {
        name: 'extractMin',
        signature: 'heap.extractMin() -> T | Null',
        summary: 'Removes and returns the minimum element from the heap.',
        parameters: [],
        returnType: { type: 'T | Null', description: 'Smallest item or null.' },
        timeComplexity: 'O(log n)',
        spaceComplexity: 'O(1)',
        description: 'Extracts root and sifts down.',
        example: 'print(h.extractMin()); // 10'
      },
      {
        name: 'peek',
        signature: 'heap.peek() -> T | Null',
        summary: 'Returns the minimum element without removing it.',
        parameters: [],
        returnType: { type: 'T | Null', description: 'Smallest item or null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Inspects root.',
        example: 'print(h.peek()); // 20'
      },
      {
        name: 'size',
        signature: 'heap.size() -> Int',
        summary: 'Returns count of items in heap.',
        parameters: [],
        returnType: { type: 'Int', description: 'Size.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Returns size.',
        example: 'print(h.size());'
      }
    ]
  },
  {
    id: 'dsa-maxheap',
    title: 'MaxHeap',
    category: 'Data Structures',
    subCategory: 'Trees & Heaps',
    breadcrumbs: ['References', 'Data Structures', 'MaxHeap'],
    summary: 'A binary max-heap maintaining the maximum element at the root with O(log n) operations.',
    badge: 'Standard Library',
    constructorSyntax: 'let heap = MaxHeap();',
    constructorParameters: [],
    timeComplexity: 'Insert: O(log n) | ExtractMax: O(log n) | Peek: O(1)',
    spaceComplexity: 'O(n)',
    overview: 'MaxHeap maintains the largest value at index 0.',
    methods: [
      {
        name: 'insert',
        signature: 'heap.insert(item: T) -> Null',
        summary: 'Inserts element into max heap.',
        parameters: [{ name: 'item', type: 'T', description: 'Item to insert.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(log n)',
        spaceComplexity: 'O(1)',
        description: 'Pushes and heapifies up.',
        example: 'let h = MaxHeap();\nh.insert(10);\nh.insert(50);\nh.insert(20);'
      },
      {
        name: 'extractMax',
        signature: 'heap.extractMax() -> T | Null',
        summary: 'Removes and returns maximum item.',
        parameters: [],
        returnType: { type: 'T | Null', description: 'Max item or null.' },
        timeComplexity: 'O(log n)',
        spaceComplexity: 'O(1)',
        description: 'Pops root and heapifies down.',
        example: 'print(h.extractMax()); // 50'
      }
    ]
  },
  {
    id: 'dsa-linkedlist',
    title: 'LinkedList',
    category: 'Data Structures',
    subCategory: 'Sequential Collections',
    breadcrumbs: ['References', 'Data Structures', 'LinkedList'],
    summary: 'A linear data structure of linked nodes with O(1) prepend and append.',
    badge: 'Standard Library',
    constructorSyntax: 'let list = LinkedList();',
    constructorParameters: [],
    timeComplexity: 'Prepend: O(1) | Append: O(1) | Search: O(n)',
    spaceComplexity: 'O(n)',
    overview: 'Singly linked list providing fast head and tail modifications.',
    methods: [
      {
        name: 'append',
        signature: 'list.append(item: T) -> Null',
        summary: 'Adds item to tail.',
        parameters: [{ name: 'item', type: 'T', description: 'Value to append.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Appends to end.',
        example: 'list.append(10);'
      },
      {
        name: 'prepend',
        signature: 'list.prepend(item: T) -> Null',
        summary: 'Adds item to head.',
        parameters: [{ name: 'item', type: 'T', description: 'Value to prepend.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Inserts at head.',
        example: 'list.prepend(5);'
      },
      {
        name: 'delete',
        signature: 'list.delete(item: T) -> Bool',
        summary: 'Deletes first occurrence of item.',
        parameters: [{ name: 'item', type: 'T', description: 'Value to delete.' }],
        returnType: { type: 'Bool', description: 'True if deleted.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(1)',
        description: 'Unlinks node.',
        example: 'list.delete(5);'
      },
      {
        name: 'reverse',
        signature: 'list.reverse() -> Null',
        summary: 'Reverses list in place.',
        parameters: [],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(1)',
        description: 'Inverts node pointers.',
        example: 'list.reverse();'
      }
    ]
  },
  {
    id: 'dsa-doublylinkedlist',
    title: 'DoublyLinkedList',
    category: 'Data Structures',
    subCategory: 'Sequential Collections',
    breadcrumbs: ['References', 'Data Structures', 'DoublyLinkedList'],
    summary: 'A bidirectional linked list where each node points to both its predecessor and successor.',
    badge: 'Standard Library',
    constructorSyntax: 'let dlist = DoublyLinkedList();',
    constructorParameters: [],
    timeComplexity: 'Prepend: O(1) | Append: O(1) | DeleteNode: O(1)',
    spaceComplexity: 'O(n)',
    overview: 'Facilitates fast bidirectional traversal and node splice.',
    methods: [
      {
        name: 'append',
        signature: 'dlist.append(item: T) -> Null',
        summary: 'Appends item to tail.',
        parameters: [{ name: 'item', type: 'T', description: 'Value to append.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Appends to end.',
        example: 'dlist.append("A");'
      },
      {
        name: 'prepend',
        signature: 'dlist.prepend(item: T) -> Null',
        summary: 'Prepends item to head.',
        parameters: [{ name: 'item', type: 'T', description: 'Value to prepend.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Inserts at head.',
        example: 'dlist.prepend("Z");'
      }
    ]
  },
  {
    id: 'dsa-bst',
    title: 'BinarySearchTree (BST)',
    category: 'Data Structures',
    subCategory: 'Trees & Heaps',
    breadcrumbs: ['References', 'Data Structures', 'BST'],
    summary: 'A node-based binary tree data structure with key ordering property for fast search, insertion, and ordered traversal.',
    badge: 'Standard Library',
    constructorSyntax: 'let bst = BST();\nlet tree = BinarySearchTree();',
    constructorParameters: [],
    timeComplexity: 'Search: O(log n) avg, O(n) worst | Insert: O(log n) avg | Delete: O(log n) avg',
    spaceComplexity: 'O(n)',
    overview: 'Maintains left subtree < root < right subtree. Supports in-order, pre-order, and post-order depth-first traversals.',
    methods: [
      {
        name: 'insert',
        signature: 'bst.insert(value: T) -> Null',
        summary: 'Inserts a value into the BST preserving search tree invariant.',
        parameters: [{ name: 'value', type: 'T', description: 'Value to insert.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(log n) average',
        spaceComplexity: 'O(1)',
        description: 'Traverses to appropriate leaf location.',
        example: 'let bst = BST();\nbst.insert(50);\nbst.insert(30);\nbst.insert(70);'
      },
      {
        name: 'contains',
        signature: 'bst.contains(value: T) -> Bool',
        summary: 'Checks if value exists in tree.',
        parameters: [{ name: 'value', type: 'T', description: 'Value to search.' }],
        returnType: { type: 'Bool', description: 'True if found.' },
        timeComplexity: 'O(log n) average',
        spaceComplexity: 'O(1)',
        description: 'Binary search over tree nodes.',
        example: 'print(bst.contains(30)); // true'
      },
      {
        name: 'inorder',
        signature: 'bst.inorder() -> Array<T>',
        summary: 'Returns an array of elements in sorted ascending order.',
        parameters: [],
        returnType: { type: 'Array<T>', description: 'Sorted array.' },
        timeComplexity: 'O(n)',
        spaceComplexity: 'O(n)',
        description: 'In-order depth-first traversal (Left, Root, Right).',
        example: 'print(bst.inorder()); // [30, 50, 70]'
      },
      {
        name: 'min',
        signature: 'bst.min() -> T | Null',
        summary: 'Returns minimum element.',
        parameters: [],
        returnType: { type: 'T | Null', description: 'Minimum value.' },
        timeComplexity: 'O(log n)',
        spaceComplexity: 'O(1)',
        description: 'Leftmost node.',
        example: 'print(bst.min()); // 30'
      },
      {
        name: 'max',
        signature: 'bst.max() -> T | Null',
        summary: 'Returns maximum element.',
        parameters: [],
        returnType: { type: 'T | Null', description: 'Maximum value.' },
        timeComplexity: 'O(log n)',
        spaceComplexity: 'O(1)',
        description: 'Rightmost node.',
        example: 'print(bst.max()); // 70'
      }
    ]
  },
  {
    id: 'dsa-avl',
    title: 'AVLTree',
    category: 'Data Structures',
    subCategory: 'Trees & Heaps',
    breadcrumbs: ['References', 'Data Structures', 'AVLTree'],
    summary: 'A strictly self-balancing binary search tree guaranteeing O(log n) worst-case time complexity.',
    badge: 'Standard Library',
    constructorSyntax: 'let avl = AVLTree();',
    constructorParameters: [],
    timeComplexity: 'Search: O(log n) | Insert: O(log n) | Delete: O(log n)',
    spaceComplexity: 'O(n)',
    overview: 'Maintains height balance factor within {-1, 0, +1} through single and double tree rotations.',
    methods: [
      {
        name: 'insert',
        signature: 'avl.insert(value: T) -> Null',
        summary: 'Inserts value and rebalances tree via rotations.',
        parameters: [{ name: 'value', type: 'T', description: 'Value to insert.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(log n)',
        spaceComplexity: 'O(1)',
        description: 'Balances tree using LL, RR, LR, or RL rotation.',
        example: 'let avl = AVLTree();\navl.insert(10);\navl.insert(20);\navl.insert(30);'
      },
      {
        name: 'isBalanced',
        signature: 'avl.isBalanced() -> Bool',
        summary: 'Verifies the AVL balance property.',
        parameters: [],
        returnType: { type: 'Bool', description: 'True if balanced.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Checks balance factor.',
        example: 'print(avl.isBalanced()); // true'
      }
    ]
  },
  {
    id: 'dsa-redblack',
    title: 'RedBlackTree',
    category: 'Data Structures',
    subCategory: 'Trees & Heaps',
    breadcrumbs: ['References', 'Data Structures', 'RedBlackTree'],
    summary: 'A self-balancing binary search tree with node color rules ensuring height bounded by 2 log(n + 1).',
    badge: 'Standard Library',
    constructorSyntax: 'let rb = RedBlackTree();',
    constructorParameters: [],
    timeComplexity: 'Search: O(log n) | Insert: O(log n) | Delete: O(log n)',
    spaceComplexity: 'O(n)',
    overview: 'Balances with color bits and rotations. Foundation for industrial map/set implementations.',
    methods: [
      {
        name: 'insert',
        signature: 'rb.insert(value: T) -> Null',
        summary: 'Inserts value into Red-Black tree.',
        parameters: [{ name: 'value', type: 'T', description: 'Value to insert.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(log n)',
        spaceComplexity: 'O(1)',
        description: 'Maintains black-height invariant.',
        example: 'rb.insert(100);'
      }
    ]
  },
  {
    id: 'dsa-trie',
    title: 'Trie (Prefix Tree)',
    category: 'Data Structures',
    subCategory: 'String & Prefix Structures',
    breadcrumbs: ['References', 'Data Structures', 'Trie'],
    summary: 'A radix/prefix search tree specialized in fast retrieval of strings and auto-completion prefixes.',
    badge: 'Standard Library',
    constructorSyntax: 'let trie = Trie();',
    constructorParameters: [],
    timeComplexity: 'Insert: O(m) | Search: O(m) | StartsWith: O(m) where m = word length',
    spaceComplexity: 'O(alphabet_size * m * n)',
    overview: 'Optimized for dictionary lookups, spell checking, IP routing, and auto-complete dropdowns.',
    methods: [
      {
        name: 'insert',
        signature: 'trie.insert(word: String) -> Null',
        summary: 'Inserts a string word into the trie.',
        parameters: [{ name: 'word', type: 'String', description: 'Word to insert.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(m)',
        spaceComplexity: 'O(m)',
        description: 'Creates nodes for each character if absent.',
        example: 'let trie = Trie();\ntrie.insert("apple");\ntrie.insert("app");'
      },
      {
        name: 'search',
        signature: 'trie.search(word: String) -> Bool',
        summary: 'Checks if exact word exists in trie.',
        parameters: [{ name: 'word', type: 'String', description: 'Word to search.' }],
        returnType: { type: 'Bool', description: 'True if word is present.' },
        timeComplexity: 'O(m)',
        spaceComplexity: 'O(1)',
        description: 'Finds exact word.',
        example: 'print(trie.search("app")); // true\nprint(trie.search("ap")); // false'
      },
      {
        name: 'startsWith',
        signature: 'trie.startsWith(prefix: String) -> Bool',
        summary: 'Checks if any word in the trie starts with the given prefix.',
        parameters: [{ name: 'prefix', type: 'String', description: 'Prefix to test.' }],
        returnType: { type: 'Bool', description: 'True if prefix exists.' },
        timeComplexity: 'O(m)',
        spaceComplexity: 'O(1)',
        description: 'Tests prefix existence.',
        example: 'print(trie.startsWith("ap")); // true'
      },
      {
        name: 'wordsWithPrefix',
        signature: 'trie.wordsWithPrefix(prefix: String) -> Array<String>',
        summary: 'Returns all words matching prefix.',
        parameters: [{ name: 'prefix', type: 'String', description: 'Prefix string.' }],
        returnType: { type: 'Array<String>', description: 'Array of matching words.' },
        timeComplexity: 'O(p + k)',
        spaceComplexity: 'O(k)',
        description: 'Auto-completion query.',
        example: 'print(trie.wordsWithPrefix("app")); // ["app", "apple"]'
      }
    ]
  },
  {
    id: 'dsa-graph',
    title: 'Graph',
    category: 'Data Structures',
    subCategory: 'Graph Structures',
    breadcrumbs: ['References', 'Data Structures', 'Graph'],
    summary: 'A versatile graph structure supporting directed, undirected, weighted edges with BFS, DFS, and topological sort algorithms.',
    badge: 'Standard Library',
    constructorSyntax: 'let g = Graph();',
    constructorParameters: [],
    timeComplexity: 'AddVertex: O(1) | AddEdge: O(1) | BFS/DFS: O(V + E)',
    spaceComplexity: 'O(V + E)',
    overview: 'Adjacency list representation suitable for pathfinding, topological sorting, dependency analysis, and network flows.',
    methods: [
      {
        name: 'addVertex',
        signature: 'graph.addVertex(v: T) -> Null',
        summary: 'Adds a vertex node to the graph.',
        parameters: [{ name: 'v', type: 'T', description: 'Vertex identifier.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Registers vertex.',
        example: 'let g = Graph();\ng.addVertex("A");\ng.addVertex("B");'
      },
      {
        name: 'addEdge',
        signature: 'graph.addEdge(u: T, v: T, weight?: Float) -> Null',
        summary: 'Adds an edge between vertex u and vertex v.',
        parameters: [
          { name: 'u', type: 'T', description: 'Source vertex.' },
          { name: 'v', type: 'T', description: 'Destination vertex.' },
          { name: 'weight', type: 'Float', description: 'Optional edge weight. Defaults to 1.0.', optional: true }
        ],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Connects two vertices.',
        example: 'g.addEdge("A", "B", 1.0);'
      },
      {
        name: 'getNeighbors',
        signature: 'graph.getNeighbors(v: T) -> Array<T>',
        summary: 'Returns adjacent neighbors of vertex v.',
        parameters: [{ name: 'v', type: 'T', description: 'Vertex identifier.' }],
        returnType: { type: 'Array<T>', description: 'List of neighbor vertices.' },
        timeComplexity: 'O(deg(v))',
        spaceComplexity: 'O(deg(v))',
        description: 'Retrieves outgoing neighbor list.',
        example: 'print(g.getNeighbors("A")); // ["B"]'
      },
      {
        name: 'bfs',
        signature: 'graph.bfs(start: T) -> Array<T>',
        summary: 'Performs breadth-first search starting from vertex.',
        parameters: [{ name: 'start', type: 'T', description: 'Starting vertex.' }],
        returnType: { type: 'Array<T>', description: 'Traversal order of vertices.' },
        timeComplexity: 'O(V + E)',
        spaceComplexity: 'O(V)',
        description: 'Level-order traversal using queue.',
        example: 'print(g.bfs("A"));'
      },
      {
        name: 'dfs',
        signature: 'graph.dfs(start: T) -> Array<T>',
        summary: 'Performs depth-first search starting from vertex.',
        parameters: [{ name: 'start', type: 'T', description: 'Starting vertex.' }],
        returnType: { type: 'Array<T>', description: 'Traversal order of vertices.' },
        timeComplexity: 'O(V + E)',
        spaceComplexity: 'O(V)',
        description: 'Depth-first search using stack.',
        example: 'print(g.dfs("A"));'
      },
      {
        name: 'hasCycle',
        signature: 'graph.hasCycle() -> Bool',
        summary: 'Detects whether the graph contains a cycle.',
        parameters: [],
        returnType: { type: 'Bool', description: 'True if cycle exists.' },
        timeComplexity: 'O(V + E)',
        spaceComplexity: 'O(V)',
        description: 'Cycle detection using visited state coloring.',
        example: 'print(g.hasCycle());'
      }
    ]
  },
  {
    id: 'dsa-lru',
    title: 'LRUCache',
    category: 'Data Structures',
    subCategory: 'Caches & Buffers',
    breadcrumbs: ['References', 'Data Structures', 'LRUCache'],
    summary: 'A Least Recently Used cache with fixed capacity and O(1) get and put operations.',
    badge: 'Standard Library',
    constructorSyntax: 'let cache = LRUCache(capacity: Int);',
    constructorParameters: [
      { name: 'capacity', type: 'Int', description: 'Maximum number of items the cache holds before evicting oldest item.' }
    ],
    timeComplexity: 'Get: O(1) | Put: O(1)',
    spaceComplexity: 'O(capacity)',
    overview: 'Combines a hash table with a doubly-linked list to achieve O(1) eviction of the least recently accessed item.',
    methods: [
      {
        name: 'put',
        signature: 'cache.put(key: K, value: V) -> Null',
        summary: 'Inserts or updates key. If capacity exceeded, evicts least recently accessed entry.',
        parameters: [
          { name: 'key', type: 'K', description: 'Entry key.' },
          { name: 'value', type: 'V', description: 'Entry value.' }
        ],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Inserts and marks as most recently used.',
        example: 'let lru = LRUCache(2);\nlru.put("a", 1);\nlru.put("b", 2);\nlru.put("c", 3); // evicts "a"'
      },
      {
        name: 'get',
        signature: 'cache.get(key: K) -> V | Null',
        summary: 'Retrieves value and moves key to most recently used position.',
        parameters: [{ name: 'key', type: 'K', description: 'Key to find.' }],
        returnType: { type: 'V | Null', description: 'Value or null if not in cache.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Updates recency on access.',
        example: 'print(lru.get("a")); // null\nprint(lru.get("b")); // 2'
      },
      {
        name: 'size',
        signature: 'cache.size() -> Int',
        summary: 'Returns current number of cached items.',
        parameters: [],
        returnType: { type: 'Int', description: 'Item count.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Returns size.',
        example: 'print(lru.size());'
      }
    ]
  },
  {
    id: 'dsa-lfu',
    title: 'LFUCache',
    category: 'Data Structures',
    subCategory: 'Caches & Buffers',
    breadcrumbs: ['References', 'Data Structures', 'LFUCache'],
    summary: 'A Least Frequently Used cache that evicts items with the lowest access count.',
    badge: 'Standard Library',
    constructorSyntax: 'let lfu = LFUCache(capacity: Int);',
    constructorParameters: [{ name: 'capacity', type: 'Int', description: 'Maximum entries before eviction.' }],
    timeComplexity: 'Get: O(1) avg | Put: O(1) avg',
    spaceComplexity: 'O(capacity)',
    overview: 'Tracks access counts per key to retain hottest entries.',
    methods: [
      {
        name: 'put',
        signature: 'lfu.put(key: K, value: V) -> Null',
        summary: 'Inserts key or updates value and increments frequency.',
        parameters: [{ name: 'key', type: 'K', description: 'Key.' }, { name: 'value', type: 'V', description: 'Value.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Stores item.',
        example: 'lfu.put("x", 100);'
      },
      {
        name: 'get',
        signature: 'lfu.get(key: K) -> V | Null',
        summary: 'Returns value and increments frequency counter.',
        parameters: [{ name: 'key', type: 'K', description: 'Key.' }],
        returnType: { type: 'V | Null', description: 'Value or null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Increments frequency.',
        example: 'print(lfu.get("x"));'
      }
    ]
  },
  {
    id: 'dsa-circularbuffer',
    title: 'CircularBuffer (Ring Buffer)',
    category: 'Data Structures',
    subCategory: 'Caches & Buffers',
    breadcrumbs: ['References', 'Data Structures', 'CircularBuffer'],
    summary: 'A fixed-size circular ring buffer that overwrites oldest entries when full.',
    badge: 'Standard Library',
    constructorSyntax: 'let cb = CircularBuffer(capacity: Int);',
    constructorParameters: [{ name: 'capacity', type: 'Int', description: 'Buffer capacity.' }],
    timeComplexity: 'Push: O(1) | Pop: O(1)',
    spaceComplexity: 'O(capacity)',
    overview: 'Circular buffer commonly used in streaming audio, socket buffers, and sliding window metrics.',
    methods: [
      {
        name: 'push',
        signature: 'cb.push(item: T) -> Null',
        summary: 'Pushes item, overwriting oldest if capacity is reached.',
        parameters: [{ name: 'item', type: 'T', description: 'Item.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Circular write.',
        example: 'cb.push(1);'
      },
      {
        name: 'pop',
        signature: 'cb.pop() -> T | Null',
        summary: 'Removes and returns oldest item.',
        parameters: [],
        returnType: { type: 'T | Null', description: 'Item.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Circular read.',
        example: 'print(cb.pop());'
      }
    ]
  },
  {
    id: 'dsa-bloomfilter',
    title: 'BloomFilter',
    category: 'Data Structures',
    subCategory: 'Probabilistic Structures',
    breadcrumbs: ['References', 'Data Structures', 'BloomFilter'],
    summary: 'A space-efficient probabilistic data structure used to test whether an element is a member of a set with zero false negatives.',
    badge: 'Standard Library',
    constructorSyntax: 'let bf = BloomFilter(bitSize: Int);',
    constructorParameters: [{ name: 'bitSize', type: 'Int', description: 'Bit array size.' }],
    timeComplexity: 'Add: O(k) | MightContain: O(k) where k = hash count',
    spaceComplexity: 'O(m) bits',
    overview: 'Returns either "possibly in set" or "definitely not in set". Commonly used in databases (LSM trees, Cassandra) to avoid expensive disk lookups.',
    methods: [
      {
        name: 'add',
        signature: 'bf.add(item: Any) -> Null',
        summary: 'Hashes item and sets corresponding bits.',
        parameters: [{ name: 'item', type: 'Any', description: 'Item to add.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(k)',
        spaceComplexity: 'O(1)',
        description: 'Sets bit positions.',
        example: 'bf.add("user@example.com");'
      },
      {
        name: 'mightContain',
        signature: 'bf.mightContain(item: Any) -> Bool',
        summary: 'Returns false if item is definitely not in set; true if it might be.',
        parameters: [{ name: 'item', type: 'Any', description: 'Item to test.' }],
        returnType: { type: 'Bool', description: 'Probabilistic membership test.' },
        timeComplexity: 'O(k)',
        spaceComplexity: 'O(1)',
        description: 'Tests bit positions.',
        example: 'if (bf.mightContain("user@example.com")) { /* check db */ }'
      }
    ]
  },
  {
    id: 'dsa-disjointset',
    title: 'DisjointSet (Union-Find)',
    category: 'Data Structures',
    subCategory: 'Graph Structures',
    breadcrumbs: ['References', 'Data Structures', 'DisjointSet'],
    summary: 'A disjoint-set data structure with union-by-rank and path compression achieving near O(1) amortized operations (Inverse Ackermann α(n)).',
    badge: 'Standard Library',
    constructorSyntax: 'let uf = DisjointSet(size: Int);\nlet dsu = UnionFind(size: Int);',
    constructorParameters: [{ name: 'size', type: 'Int', description: 'Number of elements.' }],
    timeComplexity: 'Find: O(α(n)) ≈ O(1) | Union: O(α(n)) ≈ O(1)',
    spaceComplexity: 'O(n)',
    overview: 'Essential for Kruskal minimum spanning tree, connected components, and cycle detection.',
    methods: [
      {
        name: 'find',
        signature: 'uf.find(i: Int) -> Int',
        summary: 'Finds the representative root of element i with path compression.',
        parameters: [{ name: 'i', type: 'Int', description: 'Element index.' }],
        returnType: { type: 'Int', description: 'Root representative.' },
        timeComplexity: 'O(α(n))',
        spaceComplexity: 'O(1)',
        description: 'Compresses path.',
        example: 'let root = uf.find(5);'
      },
      {
        name: 'union',
        signature: 'uf.union(i: Int, j: Int) -> Bool',
        summary: 'Merges the sets containing element i and element j using union-by-rank.',
        parameters: [{ name: 'i', type: 'Int', description: 'First element.' }, { name: 'j', type: 'Int', description: 'Second element.' }],
        returnType: { type: 'Bool', description: 'True if merged, false if already in same set.' },
        timeComplexity: 'O(α(n))',
        spaceComplexity: 'O(1)',
        description: 'Merges components.',
        example: 'uf.union(1, 2);'
      },
      {
        name: 'connected',
        signature: 'uf.connected(i: Int, j: Int) -> Bool',
        summary: 'Returns true if elements i and j belong to the same component.',
        parameters: [{ name: 'i', type: 'Int', description: 'First element.' }, { name: 'j', type: 'Int', description: 'Second element.' }],
        returnType: { type: 'Bool', description: 'True if connected.' },
        timeComplexity: 'O(α(n))',
        spaceComplexity: 'O(1)',
        description: 'Checks connectivity.',
        example: 'print(uf.connected(1, 2)); // true'
      }
    ]
  },
  {
    id: 'dsa-segmenttree',
    title: 'SegmentTree',
    category: 'Data Structures',
    subCategory: 'Range Query Structures',
    breadcrumbs: ['References', 'Data Structures', 'SegmentTree'],
    summary: 'A tree data structure for storing intervals or segments, allowing querying which of the stored segments contain a given point with O(log n) updates and range queries.',
    badge: 'Standard Library',
    constructorSyntax: 'let seg = SegmentTree([1, 3, 5, 7, 9, 11]);',
    constructorParameters: [{ name: 'array', type: 'Array<Int>', description: 'Initial array of values.' }],
    timeComplexity: 'Build: O(n) | Query: O(log n) | Update: O(log n)',
    spaceComplexity: 'O(n)',
    overview: 'Optimized for dynamic range sum, range minimum, and range maximum queries.',
    methods: [
      {
        name: 'query',
        signature: 'seg.query(left: Int, right: Int) -> Int',
        summary: 'Returns sum of elements in range [left, right].',
        parameters: [{ name: 'left', type: 'Int', description: 'Start index.' }, { name: 'right', type: 'Int', description: 'End index.' }],
        returnType: { type: 'Int', description: 'Aggregated sum.' },
        timeComplexity: 'O(log n)',
        spaceComplexity: 'O(1)',
        description: 'Range sum.',
        example: 'print(seg.query(1, 3)); // 3 + 5 + 7 = 15'
      },
      {
        name: 'update',
        signature: 'seg.update(index: Int, value: Int) -> Null',
        summary: 'Updates element at index and propagates to root.',
        parameters: [{ name: 'index', type: 'Int', description: 'Index.' }, { name: 'value', type: 'Int', description: 'New value.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(log n)',
        spaceComplexity: 'O(1)',
        description: 'Updates value.',
        example: 'seg.update(2, 10);'
      }
    ]
  },
  {
    id: 'dsa-fenwicktree',
    title: 'FenwickTree (Binary Indexed Tree)',
    category: 'Data Structures',
    subCategory: 'Range Query Structures',
    breadcrumbs: ['References', 'Data Structures', 'FenwickTree'],
    summary: 'A Binary Indexed Tree (BIT) supporting O(log n) point updates and prefix sum queries with minimal space overhead.',
    badge: 'Standard Library',
    constructorSyntax: 'let bit = FenwickTree(size: Int);',
    constructorParameters: [{ name: 'size', type: 'Int', description: 'Number of elements.' }],
    timeComplexity: 'Update: O(log n) | Query: O(log n)',
    spaceComplexity: 'O(n)',
    overview: 'Uses bitwise two-complement trick (idx & -idx) to navigate cumulative sums.',
    methods: [
      {
        name: 'update',
        signature: 'bit.update(index: Int, delta: Int) -> Null',
        summary: 'Adds delta to element at index.',
        parameters: [{ name: 'index', type: 'Int', description: 'Zero-based index.' }, { name: 'delta', type: 'Int', description: 'Value to add.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(log n)',
        spaceComplexity: 'O(1)',
        description: 'Updates prefix trees.',
        example: 'bit.update(0, 5);'
      },
      {
        name: 'query',
        signature: 'bit.query(index: Int) -> Int',
        summary: 'Returns prefix sum from 0 to index inclusive.',
        parameters: [{ name: 'index', type: 'Int', description: 'Zero-based index.' }],
        returnType: { type: 'Int', description: 'Cumulative sum.' },
        timeComplexity: 'O(log n)',
        spaceComplexity: 'O(1)',
        description: 'Prefix query.',
        example: 'print(bit.query(0)); // 5'
      }
    ]
  },
  {
    id: 'dsa-bitset',
    title: 'BitSet',
    category: 'Data Structures',
    subCategory: 'Bit Manipulation Structures',
    breadcrumbs: ['References', 'Data Structures', 'BitSet'],
    summary: 'A compact vector of boolean flags packed into 64-bit words with fast bitwise operations.',
    badge: 'Standard Library',
    constructorSyntax: 'let bs = BitSet(bitCount: Int);',
    constructorParameters: [{ name: 'bitCount', type: 'Int', description: 'Number of bits.' }],
    timeComplexity: 'Set/Get/Toggle: O(1)',
    spaceComplexity: 'O(n / 64) words',
    overview: 'Ideal for memory-constrained flag tracking, sieve of Eratosthenes, and graph state masks.',
    methods: [
      {
        name: 'set',
        signature: 'bs.set(index: Int) -> Null',
        summary: 'Sets bit at index to true.',
        parameters: [{ name: 'index', type: 'Int', description: 'Bit index.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Sets bit.',
        example: 'bs.set(5);'
      },
      {
        name: 'get',
        signature: 'bs.get(index: Int) -> Bool',
        summary: 'Returns boolean state of bit at index.',
        parameters: [{ name: 'index', type: 'Int', description: 'Bit index.' }],
        returnType: { type: 'Bool', description: 'Bit value.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Reads bit.',
        example: 'print(bs.get(5)); // true'
      },
      {
        name: 'toggle',
        signature: 'bs.toggle(index: Int) -> Null',
        summary: 'Flips bit at index.',
        parameters: [{ name: 'index', type: 'Int', description: 'Bit index.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Toggles bit.',
        example: 'bs.toggle(5);'
      },
      {
        name: 'count',
        signature: 'bs.count() -> Int',
        summary: 'Returns count of set bits (population count).',
        parameters: [],
        returnType: { type: 'Int', description: 'Number of true bits.' },
        timeComplexity: 'O(n / 64)',
        spaceComplexity: 'O(1)',
        description: 'Popcount.',
        example: 'print(bs.count());'
      }
    ]
  },
  {
    id: 'dsa-skiplist',
    title: 'SkipList',
    category: 'Data Structures',
    subCategory: 'Probabilistic Structures',
    breadcrumbs: ['References', 'Data Structures', 'SkipList'],
    summary: 'A probabilistic alternative to balanced trees maintaining sorted order with O(log n) operations.',
    badge: 'Standard Library',
    constructorSyntax: 'let sl = SkipList();',
    constructorParameters: [],
    timeComplexity: 'Search: O(log n) avg | Insert: O(log n) avg | Remove: O(log n) avg',
    spaceComplexity: 'O(n)',
    overview: 'Multi-level forward pointers allowing binary search over linked list nodes.',
    methods: [
      {
        name: 'insert',
        signature: 'sl.insert(value: T) -> Null',
        summary: 'Inserts value into skip list.',
        parameters: [{ name: 'value', type: 'T', description: 'Value.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(log n)',
        spaceComplexity: 'O(1)',
        description: 'Inserts with random tower height.',
        example: 'sl.insert(10);'
      },
      {
        name: 'search',
        signature: 'sl.search(value: T) -> Bool',
        summary: 'Finds value in skip list.',
        parameters: [{ name: 'value', type: 'T', description: 'Value.' }],
        returnType: { type: 'Bool', description: 'True if found.' },
        timeComplexity: 'O(log n)',
        spaceComplexity: 'O(1)',
        description: 'Traverses levels.',
        example: 'print(sl.search(10)); // true'
      }
    ]
  },
  {
    id: 'dsa-matrix',
    title: 'Matrix',
    category: 'Data Structures',
    subCategory: 'Linear Algebra & Grids',
    breadcrumbs: ['References', 'Data Structures', 'Matrix'],
    summary: 'A 2D dense matrix grid with transpose, addition, and matrix multiplication routines.',
    badge: 'Standard Library',
    constructorSyntax: 'let m = Matrix(rows: Int, cols: Int, defaultValue?: Float);',
    constructorParameters: [
      { name: 'rows', type: 'Int', description: 'Row count.' },
      { name: 'cols', type: 'Int', description: 'Column count.' },
      { name: 'defaultValue', type: 'Float', description: 'Initial cell value. Defaults to 0.0.', optional: true }
    ],
    timeComplexity: 'Get/Set: O(1) | Transpose: O(r * c) | Multiply: O(r * c * k)',
    spaceComplexity: 'O(r * c)',
    overview: 'Dense 2D grid matrix operations for graphics, linear algebra, and DP tables.',
    methods: [
      {
        name: 'get',
        signature: 'm.get(r: Int, c: Int) -> Float',
        summary: 'Gets cell value at row r, column c.',
        parameters: [{ name: 'r', type: 'Int', description: 'Row.' }, { name: 'c', type: 'Int', description: 'Col.' }],
        returnType: { type: 'Float', description: 'Value.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Matrix cell access.',
        example: 'let val = m.get(0, 1);'
      },
      {
        name: 'set',
        signature: 'm.set(r: Int, c: Int, value: Float) -> Null',
        summary: 'Sets cell value.',
        parameters: [{ name: 'r', type: 'Int', description: 'Row.' }, { name: 'c', type: 'Int', description: 'Col.' }, { name: 'value', type: 'Float', description: 'Value.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Sets cell value.',
        example: 'm.set(0, 1, 3.14);'
      },
      {
        name: 'rows',
        signature: 'm.rows() -> Int',
        summary: 'Returns row count.',
        parameters: [],
        returnType: { type: 'Int', description: 'Row count.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Rows count.',
        example: 'print(m.rows());'
      },
      {
        name: 'cols',
        signature: 'm.cols() -> Int',
        summary: 'Returns column count.',
        parameters: [],
        returnType: { type: 'Int', description: 'Column count.' },
        timeComplexity: 'O(1)',
        spaceComplexity: 'O(1)',
        description: 'Cols count.',
        example: 'print(m.cols());'
      }
    ]
  },
  {
    id: 'dsa-sparsematrix',
    title: 'SparseMatrix',
    category: 'Data Structures',
    subCategory: 'Linear Algebra & Grids',
    breadcrumbs: ['References', 'Data Structures', 'SparseMatrix'],
    summary: 'A memory-efficient representation for large matrices where most elements are zero.',
    badge: 'Standard Library',
    constructorSyntax: 'let sm = SparseMatrix(rows: Int, cols: Int);',
    constructorParameters: [
      { name: 'rows', type: 'Int', description: 'Total row dimensions.' },
      { name: 'cols', type: 'Int', description: 'Total column dimensions.' }
    ],
    timeComplexity: 'Get/Set: O(k) where k = non-zero count',
    spaceComplexity: 'O(k)',
    overview: 'Stores only non-zero entries in coordinate format.',
    methods: [
      {
        name: 'set',
        signature: 'sm.set(r: Int, c: Int, val: Float) -> Null',
        summary: 'Stores non-zero value at (r, c).',
        parameters: [{ name: 'r', type: 'Int', description: 'Row.' }, { name: 'c', type: 'Int', description: 'Col.' }, { name: 'val', type: 'Float', description: 'Value.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(k)',
        spaceComplexity: 'O(1)',
        description: 'Stores entry.',
        example: 'sm.set(100, 200, 4.2);'
      },
      {
        name: 'get',
        signature: 'sm.get(r: Int, c: Int) -> Float',
        summary: 'Returns value at (r, c) or 0 if unset.',
        parameters: [{ name: 'r', type: 'Int', description: 'Row.' }, { name: 'c', type: 'Int', description: 'Col.' }],
        returnType: { type: 'Float', description: 'Value or 0.0.' },
        timeComplexity: 'O(k)',
        spaceComplexity: 'O(1)',
        description: 'Finds entry.',
        example: 'print(sm.get(100, 200)); // 4.2'
      }
    ]
  },
  {
    id: 'dsa-treemap',
    title: 'TreeMap',
    category: 'Data Structures',
    subCategory: 'Keyed Collections',
    breadcrumbs: ['References', 'Data Structures', 'TreeMap'],
    summary: 'A Red-Black tree backed key-value map keeping keys in sorted natural order.',
    badge: 'Standard Library',
    constructorSyntax: 'let tm = TreeMap();',
    constructorParameters: [],
    timeComplexity: 'Get: O(log n) | Set: O(log n) | Delete: O(log n)',
    spaceComplexity: 'O(n)',
    overview: 'Sorted dictionary allowing minKey(), maxKey(), and range queries.',
    methods: [
      {
        name: 'set',
        signature: 'tm.set(key: K, value: V) -> Null',
        summary: 'Inserts key and value in sorted order.',
        parameters: [{ name: 'key', type: 'K', description: 'Key.' }, { name: 'value', type: 'V', description: 'Value.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(log n)',
        spaceComplexity: 'O(1)',
        description: 'Sorted insertion.',
        example: 'tm.set(2, "two");\ntm.set(1, "one");'
      },
      {
        name: 'get',
        signature: 'tm.get(key: K) -> V | Null',
        summary: 'Retrieves value by key.',
        parameters: [{ name: 'key', type: 'K', description: 'Key.' }],
        returnType: { type: 'V | Null', description: 'Value.' },
        timeComplexity: 'O(log n)',
        spaceComplexity: 'O(1)',
        description: 'Tree search.',
        example: 'print(tm.get(1)); // "one"'
      },
      {
        name: 'minKey',
        signature: 'tm.minKey() -> K | Null',
        summary: 'Returns lowest key.',
        parameters: [],
        returnType: { type: 'K | Null', description: 'Lowest key.' },
        timeComplexity: 'O(log n)',
        spaceComplexity: 'O(1)',
        description: 'Leftmost tree key.',
        example: 'print(tm.minKey()); // 1'
      }
    ]
  },
  {
    id: 'dsa-treeset',
    title: 'TreeSet',
    category: 'Data Structures',
    subCategory: 'Keyed Collections',
    breadcrumbs: ['References', 'Data Structures', 'TreeSet'],
    summary: 'A sorted set based on a balanced binary search tree keeping elements ordered.',
    badge: 'Standard Library',
    constructorSyntax: 'let ts = TreeSet();',
    constructorParameters: [],
    timeComplexity: 'Add: O(log n) | Has: O(log n) | Delete: O(log n)',
    spaceComplexity: 'O(n)',
    overview: 'Maintains unique elements in sorted ascending order.',
    methods: [
      {
        name: 'add',
        signature: 'ts.add(val: T) -> Null',
        summary: 'Inserts value in sorted order.',
        parameters: [{ name: 'val', type: 'T', description: 'Value to add.' }],
        returnType: { type: 'Null', description: 'Returns null.' },
        timeComplexity: 'O(log n)',
        spaceComplexity: 'O(1)',
        description: 'Inserts maintaining sort.',
        example: 'ts.add(50);\nts.add(20);'
      },
      {
        name: 'min',
        signature: 'ts.min() -> T | Null',
        summary: 'Returns lowest element.',
        parameters: [],
        returnType: { type: 'T | Null', description: 'Minimum value.' },
        timeComplexity: 'O(log n)',
        spaceComplexity: 'O(1)',
        description: 'Lowest item.',
        example: 'print(ts.min()); // 20'
      },
      {
        name: 'max',
        signature: 'ts.max() -> T | Null',
        summary: 'Returns highest element.',
        parameters: [],
        returnType: { type: 'T | Null', description: 'Maximum value.' },
        timeComplexity: 'O(log n)',
        spaceComplexity: 'O(1)',
        description: 'Highest item.',
        example: 'print(ts.max()); // 50'
      }
    ]
  }
];
