use std::collections::HashMap;


/// Returns the first `n` Fibonacci numbers.
pub fn fib(n: u32) -> Vec<u32> {
    let mut a: u32 = 0;
    let mut b: u32 = 1;
    let mut out: Vec<u32> = Vec::new();
    for _ in 0..n {
        out.push(a);
        let next = a + b;
        a = b;
        b = next;
    }
    out
}

/// Returns true if `n` is a palindrome, false otherwise.
pub fn is_palindrome(n: u32) -> bool {
    let s = n.to_string();
    let rev: String = s.chars().rev().collect();
    s == rev
}

/// Returns the nth largest element in `a`, or None if it does not exist.
pub fn nthmax(n: usize, a: &[i32]) -> Option<i32> {
    if a.is_empty() {
        return None;
    }
    if n >= a.len() {
        return None;
    }
    let mut v = a.to_vec();
    v.sort();
    Some(v[v.len() - 1 - n])
}

/// Returns a one-character String containing the most frequent character in `s`.
pub fn freq(s: &str) -> String {
    if s.is_empty() {
        return String::new();
    }
    let mut counts: HashMap<char, usize> = HashMap::new();
    for ch in s.chars() {
        *counts.entry(ch).or_insert(0) += 1;
    }
    let mut best = '\0';
    let mut best_count = 0usize;
    for (ch, count) in counts {
        if count > best_count {
            best_count = count;
            best = ch;
        }
    }
    best.to_string()
}

/// Zips two slices into a HashMap, mapping arr1[i] -> arr2[i].
pub fn zip_hash(
    arr1: &[String],
    arr2: &[String],
) -> Option<HashMap<String, String>> {
    if arr1.len() != arr2.len() {
        return None;
    }
    let mut map = HashMap::new();
    for (k, v) in arr1.iter().zip(arr2.iter()) {
        map.insert(k.clone(), v.clone());
    }
    Some(map)
}

/// Converts a HashMap into a Vec of (key, value) pairs.
pub fn hash_to_array(
    map: &HashMap<String, String>,
) -> Vec<(String, String)> {
    map.iter()
        .map(|(k, v)| (k.clone(), v.clone()))
        .collect()
}


// ========================
// Part 2: PhoneBook
// ========================

/// A single phone book entry.
#[derive(Debug, Clone)]
pub struct PhoneEntry {
    pub name: String,
    pub number: String,
    pub is_listed: bool,
}

/// PhoneBook holds name/number pairs and whether each is listed.
#[derive(Debug, Default)]
pub struct PhoneBook {
    pub entries: Vec<PhoneEntry>,
}

impl PhoneBook {
    /// Constructor: create an empty PhoneBook.
    pub fn new() -> Self {
        PhoneBook {
            entries: Vec::new(),
        }
    }

    /// Attempts to add a new entry.
    ///
    /// Rules:
    /// 1. If the name already exists, return false.
    /// 2. If the number is not in the format NNN-NNN-NNNN, return false.
    /// 3. A number can be unlisted any number of times, but listed at most once.
    ///    - If the number already exists as listed, adding another listed entry
    ///      with the same number must return false.
    ///
    /// Returns true if the entry was successfully added.
    pub fn add(
        &mut self,
        name: String,
        number: String,
        is_listed: bool,
    ) -> bool {
        if self.entries.iter().any(|e| e.name == name) {
            return false;
        }
        let chars: Vec<char> = number.chars().collect();
        if chars.len() != 12 { return false; }
        if chars[3] != '-' || chars[7] != '-' { return false; }
        for (i, ch) in chars.iter().enumerate() {
            if i == 3 || i == 7 { continue; }
            if !ch.is_ascii_digit() { return false; }
        }
        if is_listed &&
            self.entries.iter().any(|e| e.number == number && e.is_listed)
        {
            return false;
        }
        self.entries.push(PhoneEntry {
            name,
            number,
            is_listed,
        });
        true
    }

    /// Looks up `name` and returns the number ONLY if the entry is listed.
    ///
    /// Otherwise returns None.
    pub fn lookup(&self, name: &str) -> Option<String> {
        self.entries
            .iter()
            .find(|e| e.name == name && e.is_listed)
            .map(|e| e.number.clone())
    }

    /// Looks up `num` and returns the associated name ONLY if the entry is listed.
    ///
    /// Otherwise returns None.
    pub fn lookup_by_num(&self, num: &str) -> Option<String> {
        self.entries
            .iter()
            .find(|e| e.number == num && e.is_listed)
            .map(|e| e.name.clone())
    }

    /// Returns all names (listed and unlisted) whose numbers begin with `areacode`.
    pub fn names_by_ac(&self, areacode: &str) -> Vec<String> {
        self.entries
            .iter()
            .filter(|e| e.number.starts_with(areacode))
            .map(|e| e.name.clone())
            .collect()
    }
}
