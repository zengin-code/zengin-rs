use include_dir::{include_dir, Dir};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, env, error::Error, path::PathBuf};

static DATA_DIR: Dir<'_> = include_dir!("$CARGO_MANIFEST_DIR/source-data/data");

/// The `Zengin` struct represents a collection of banks and their branches.
///
/// This struct provides methods to load bank and branch data from JSON files,
/// and to retrieve information about banks and branches.
pub struct Zengin {
    banks: BankMap,
}

impl Zengin {
    /// Creates a new instance of `Zengin` by loading bank and branch data from JSON files.
    ///
    /// This function reads the `banks.json` file to load bank data and then reads
    /// corresponding branch JSON files for each bank to load branch data.
    ///
    /// # Errors
    ///
    /// Returns an error if any of the JSON files cannot be read or parsed.
    ///
    /// # Examples
    /// ```
    /// use zengin::Zengin;
    /// let zengin = Zengin::new().unwrap();
    /// ```
    pub fn new() -> Result<Zengin, Box<dyn Error>> {
        let bank_file = join_paths(&["banks.json"])?;
        let mut banks = load_banks_from_file(bank_file.to_str().unwrap())?;

        for bank in banks.values_mut() {
            let branch_file = join_paths(&["branches", &format!("{}.json", bank.code)])?;
            let branches = load_branches_from_file(branch_file.to_str().unwrap())?;
            bank.branches = branches;
        }

        Ok(Zengin { banks })
    }

    /// Retrieves a reference to a bank by its code.
    ///
    /// This function takes a bank code as input and returns an `Option` containing
    /// a reference to the corresponding `Bank` if it exists.
    ///
    /// # Arguments
    ///
    /// * `code` - A string slice that holds the bank code.
    ///
    /// # Examples
    /// ```
    /// use zengin::Zengin;
    /// let zengin = Zengin::new().unwrap();
    /// if let Some(bank) = zengin.get_bank("0001") {
    ///     println!("Found bank: {}", bank.name);
    /// }
    /// ```
    pub fn get_bank(&self, code: &str) -> Option<&Bank> {
        self.banks.get(code)
    }

    fn find_banks_by<F>(&self, pattern: &str, key_extractor: F) -> Result<Vec<&Bank>, regex::Error>
    where
        F: Fn(&Bank) -> &str,
    {
        let re = regex::Regex::new(pattern)?;
        let mut matched = vec![];
        for bank in self.banks.values() {
            if re.is_match(key_extractor(bank)) {
                matched.push(bank);
            }
        }
        Ok(matched)
    }

    /// Finds banks by their name using a regular expression pattern.
    ///
    /// This function takes a regular expression pattern as input and returns a vector
    /// of references to the banks whose names match the pattern.
    ///
    /// # Arguments
    ///
    /// * `pattern` - A string slice that holds the regular expression pattern.
    ///
    /// # Errors
    ///
    /// Returns an error if the regular expression pattern is invalid.
    ///
    /// # Examples
    /// ```
    /// use zengin::Zengin;
    /// let zengin = Zengin::new().unwrap();
    /// let banks = zengin.find_banks_by_name(".*みずほ.*").unwrap();
    /// for bank in banks {
    ///     println!("Found bank: {}", bank.name);
    /// }
    /// ```
    pub fn find_banks_by_name(&self, pattern: &str) -> Result<Vec<&Bank>, regex::Error> {
        self.find_banks_by(pattern, |bank| &bank.name)
    }

    /// Finds banks by their kana name using a regular expression pattern.
    ///
    /// This function takes a regular expression pattern as input and returns a vector
    /// of references to the banks whose names match the pattern.
    ///
    /// # Arguments
    ///
    /// * `pattern` - A string slice that holds the regular expression pattern.
    ///
    /// # Errors
    ///
    /// Returns an error if the regular expression pattern is invalid.
    ///
    /// # Examples
    /// ```
    /// use zengin::Zengin;
    /// let zengin = Zengin::new().unwrap();
    /// let banks = zengin.find_banks_by_kana(".*ミズホ.*").unwrap();
    /// for bank in banks {
    ///     println!("Found bank: {}", bank.name);
    /// }
    /// ```
    pub fn find_banks_by_kana(&self, pattern: &str) -> Result<Vec<&Bank>, regex::Error> {
        self.find_banks_by(pattern, |bank| &bank.kana)
    }

    /// Finds banks by their hiragana name using a regular expression pattern.
    ///
    /// This function takes a regular expression pattern as input and returns a vector
    /// of references to the banks whose names match the pattern.
    ///
    /// # Arguments
    ///
    /// * `pattern` - A string slice that holds the regular expression pattern.
    ///
    /// # Errors
    ///
    /// Returns an error if the regular expression pattern is invalid.
    ///
    /// # Examples
    /// ```
    /// use zengin::Zengin;
    /// let zengin = Zengin::new().unwrap();
    /// let banks = zengin.find_banks_by_hira(".*みずほ.*").unwrap();
    /// for bank in banks {
    ///     println!("Found bank: {}", bank.name);
    /// }
    /// ```
    pub fn find_banks_by_hira(&self, pattern: &str) -> Result<Vec<&Bank>, regex::Error> {
        self.find_banks_by(pattern, |bank| &bank.hira)
    }

    /// Finds banks by their romanized name using a regular expression pattern.
    ///
    /// This function takes a regular expression pattern as input and returns a vector
    /// of references to the banks whose names match the pattern.
    ///
    /// # Arguments
    ///
    /// * `pattern` - A string slice that holds the regular expression pattern.
    ///
    /// # Errors
    ///
    /// Returns an error if the regular expression pattern is invalid.
    ///
    /// # Examples
    /// ```
    /// use zengin::Zengin;
    /// let zengin = Zengin::new().unwrap();
    /// let banks = zengin.find_banks_by_roma(".*mizuho.*").unwrap();
    /// for bank in banks {
    ///     println!("Found bank: {}", bank.name);
    /// }
    /// ```
    pub fn find_banks_by_roma(&self, pattern: &str) -> Result<Vec<&Bank>, regex::Error> {
        self.find_banks_by(pattern, |bank| &bank.roma)
    }

    /// Retrieves a reference to all banks.
    ///
    /// This function returns a reference to the internal `HashMap` containing all banks.
    ///
    /// # Examples
    /// ```
    /// use zengin::Zengin;
    /// let zengin = Zengin::new().unwrap();
    /// let banks = zengin.all_banks();
    /// for (code, bank) in banks {
    ///     println!("Bank code: {}, Bank name: {}", code, bank.name);
    /// }
    /// ```
    pub fn all_banks(&self) -> &BankMap {
        &self.banks
    }
}

type BranchMap = HashMap<String, Branch>;
type BankMap = HashMap<String, Bank>;

/// The `Bank` struct represents a bank with its associated branches.
///
/// This struct contains information about the bank, including its code, name,
/// kana, hiragana, and romanized name. It also holds a collection of branches
/// associated with the bank.
#[derive(Serialize, Deserialize, Debug)]
pub struct Bank {
    pub code: String,
    pub name: String,
    pub kana: String,
    pub hira: String,
    pub roma: String,

    #[serde(skip_deserializing)]
    branches: BranchMap,
}

impl Bank {
    /// Retrieves a reference to a branch by its code.
    ///
    /// This function takes a branch code as input and returns an `Option` containing
    /// a reference to the corresponding `Branch` if it exists.
    ///
    /// # Arguments
    ///
    /// * `code` - A string slice that holds the branch code.
    ///
    /// # Examples
    /// ```
    /// if let Some(branch) = bank.get_branch("001") {
    ///     println!("Found branch: {}", branch.name);
    /// }
    /// ```
    pub fn get_branch(&self, code: &str) -> Option<&Branch> {
        self.branches.get(code)
    }

    fn find_branches_by<F>(
        &self,
        pattern: &str,
        key_extractor: F,
    ) -> Result<Vec<&Branch>, regex::Error>
    where
        F: Fn(&Branch) -> &str,
    {
        let re = regex::Regex::new(pattern)?;
        let mut matched = vec![];
        for branch in self.branches.values() {
            if re.is_match(key_extractor(branch)) {
                matched.push(branch);
            }
        }
        Ok(matched)
    }

    /// Finds branches by their name using a regular expression pattern.
    ///
    /// This function takes a regular expression pattern as input and returns a vector
    /// of references to the branches whose names match the pattern.
    ///
    /// # Arguments
    ///
    /// * `pattern` - A string slice that holds the regular expression pattern.
    ///
    /// # Errors
    ///
    /// Returns an error if the regular expression pattern is invalid.
    ///
    /// # Examples
    /// ```
    /// let branches = bank.find_branches_by_name(".*東京.*").unwrap();
    /// for branch in branches {
    ///     println!("Found branch: {}", branch.name);
    /// }
    /// ```
    pub fn find_branches_by_name(&self, pattern: &str) -> Result<Vec<&Branch>, regex::Error> {
        self.find_branches_by(pattern, |branch| &branch.name)
    }

    /// Finds branches by their hiragana name using a regular expression pattern.
    ///
    /// This function takes a regular expression pattern as input and returns a vector
    /// of references to the branches whose names match the pattern.
    ///
    /// # Arguments
    ///
    /// * `pattern` - A string slice that holds the regular expression pattern.
    ///
    /// # Errors
    ///
    /// Returns an error if the regular expression pattern is invalid.
    ///
    /// # Examples
    /// ```
    /// let branches = bank.find_branches_by_hira(".*とうきよう.*").unwrap();
    /// for branch in branches {
    ///    println!("Found branch: {}", branch.name);
    /// }
    /// ```
    pub fn find_branches_by_hira(&self, pattern: &str) -> Result<Vec<&Branch>, regex::Error> {
        self.find_branches_by(pattern, |branch| &branch.hira)
    }

    /// Finds branches by their kana name using a regular expression pattern.
    ///
    /// This function takes a regular expression pattern as input and returns a vector
    /// of references to the branches whose names match the pattern.
    ///
    /// # Arguments
    ///
    /// * `pattern` - A string slice that holds the regular expression pattern.
    ///
    /// # Errors
    ///
    /// Returns an error if the regular expression pattern is invalid.
    ///
    /// # Examples
    /// ```
    /// let branches = bank.find_branches_by_kana(".*トウキヨウ.*").unwrap();
    /// for branch in branches {
    ///    println!("Found branch: {}", branch.name);
    /// }
    /// ```
    pub fn find_branches_by_kana(&self, pattern: &str) -> Result<Vec<&Branch>, regex::Error> {
        self.find_branches_by(pattern, |branch| &branch.kana)
    }

    /// Finds branches by their romanized name using a regular expression pattern.
    ///
    /// This function takes a regular expression pattern as input and returns a vector
    /// of references to the branches whose names match the pattern.
    ///
    /// # Arguments
    ///
    /// * `pattern` - A string slice that holds the regular expression pattern.
    ///
    /// # Errors
    ///
    /// Returns an error if the regular expression pattern is invalid.
    ///
    /// # Examples
    /// ```
    /// let branches = bank.find_branches_by_roma(".*toukiyou.*").unwrap();
    /// for branch in branches {
    ///    println!("Found branch: {}", branch.name);
    /// }
    /// ```
    pub fn find_branches_by_roma(&self, pattern: &str) -> Result<Vec<&Branch>, regex::Error> {
        self.find_branches_by(pattern, |branch| &branch.roma)
    }

    /// Retrieves a reference to all branches.
    ///
    /// This function returns a reference to the internal `HashMap` containing all branches.
    ///
    /// # Examples
    /// ```
    /// let branches = bank.all_branches();
    /// for (code, branch) in branches {
    ///     println!("Branch code: {}, Branch name: {}", code, branch.name);
    /// }
    /// ```
    pub fn all_branches(&self) -> &BranchMap {
        &self.branches
    }
}

/// The `Branch` struct represents a branch of a bank.
///
/// This struct contains information about the branch, including its code, name,
/// kana, hiragana, and romanized name.
#[derive(Serialize, Deserialize, Debug)]
pub struct Branch {
    pub code: String,
    pub name: String,
    pub kana: String,
    pub hira: String,
    pub roma: String,
}

fn parse_banks(json_data: &str) -> std::result::Result<BankMap, Box<dyn Error>> {
    let bank_map = serde_json::from_str(json_data)?;
    Ok(bank_map)
}

fn parse_branches(json_data: &str) -> std::result::Result<BranchMap, Box<dyn Error>> {
    let branch_map = serde_json::from_str(json_data)?;
    Ok(branch_map)
}

fn load_banks_from_file(file_path: &str) -> std::result::Result<BankMap, Box<dyn Error>> {
    let json_data = read_data_file(file_path)?;
    let banks = parse_banks(&json_data)?;
    Ok(banks)
}

fn load_branches_from_file(file_path: &str) -> std::result::Result<BranchMap, Box<dyn Error>> {
    let json_data = read_data_file(file_path)?;
    parse_branches(&json_data)
}

fn read_data_file(file_path: &str) -> std::result::Result<String, Box<dyn Error>> {
    let data = DATA_DIR.get_file(file_path).unwrap();
    let data_str = std::str::from_utf8(data.contents())?;
    Ok(data_str.to_string())
}

fn join_paths(parts: &[&str]) -> Result<PathBuf, Box<dyn std::error::Error>> {
    let mut path = env::current_dir()?;
    for part in parts {
        path.push(part);
    }
    Ok(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_bank_data() -> &'static str {
        r#"{
            "0001":{
                "code":"0001",
                "name":"みずほ",
                "kana":"ミズホ",
                "hira":"みずほ",
                "roma":"mizuho"
            },
            "0005":{
                "code":"0005",
                "name":"三菱ＵＦＪ",
                "kana":"ミツビシユ－エフジエイ",
                "hira":"みつびしゆ－えふじえい",
                "roma":"mitsubishiyu-efujiei"
            }
        }"#
    }

    fn sample_branch_data() -> &'static str {
        r#"{
            "001":{
                "code":"001",
                "name":"東京営業部",
                "kana":"トウキヨウ",
                "hira":"とうきよう",
                "roma":"toukiyou"
            }
        }"#
    }

    #[test]
    fn test_parse_banks() {
        let json_data = sample_bank_data();
        let banks = parse_banks(json_data).unwrap();
        assert_eq!(banks["0001"].name, "みずほ");
        assert_eq!(banks["0005"].name, "三菱ＵＦＪ");
    }

    #[test]
    fn test_parse_branches() {
        let json_data = sample_branch_data();
        let branches = parse_branches(json_data).unwrap();
        assert_eq!(branches["001"].name, "東京営業部");
    }

    #[test]
    fn test_zengin_new() {
        let zengin = Zengin::new().unwrap();
        assert_eq!(zengin.banks["0001"].name, "みずほ");
    }
}
