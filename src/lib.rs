use serde::{Deserialize, Serialize};
use std::{collections::HashMap, error::Error};

pub struct Zengin {
    banks: BankMap,
}

impl Zengin {
    pub fn new() -> Zengin {
        let current_dir = std::env::current_dir().unwrap();
        let current_dir_str = current_dir.to_str().unwrap();
        let bank_file_path = format!("{}/{}", current_dir_str, "source-data/data/banks.json",);
        let bank_file = std::path::Path::new(bank_file_path.as_str());
        let mut banks = load_banks_from_file(&bank_file.to_str().unwrap()).unwrap();

        for bank in banks.iter_mut() {
            let current_dir_str = current_dir.to_str().unwrap();
            let branch_file_path = format!(
                "{}/source-data/data/branches/{}.json",
                current_dir_str,
                bank.1.code.as_str()
            );
            let path = std::path::Path::new(&branch_file_path);

            let branches = load_branches_from_file(path.to_str().unwrap()).unwrap();
            bank.1.branches = branches;
        }

        Zengin { banks }
    }

    pub fn get_bank(&self, code: &str) -> Option<&Bank> {
        self.banks.get(code)
    }

    pub fn find_banks_by_name(&self, pattern: &str) -> Result<Vec<&Bank>, regex::Error> {
        let re = regex::Regex::new(pattern)?;
        let mut matched = vec![];
        for bank in self.banks.values() {
            if re.is_match(&bank.name) {
                matched.push(bank)
            }
        }
        Ok(matched)
    }

    pub fn find_banks_by_kana(&self, pattern: &str) -> Result<Vec<&Bank>, regex::Error> {
        let re = regex::Regex::new(pattern)?;
        let mut matched = vec![];
        for bank in self.banks.values() {
            if re.is_match(&bank.kana) {
                matched.push(bank)
            }
        }
        Ok(matched)
    }

    pub fn find_banks_by_hira(&self, pattern: &str) -> Result<Vec<&Bank>, regex::Error> {
        let re = regex::Regex::new(pattern)?;
        let mut matched = vec![];
        for bank in self.banks.values() {
            if re.is_match(&bank.hira) {
                matched.push(bank)
            }
        }
        Ok(matched)
    }

    pub fn find_banks_by_roma(&self, pattern: &str) -> Result<Vec<&Bank>, regex::Error> {
        let re = regex::Regex::new(pattern)?;
        let mut matched = vec![];
        for bank in self.banks.values() {
            if re.is_match(&bank.roma) {
                matched.push(bank)
            }
        }
        Ok(matched)
    }

    pub fn all_banks(&self) -> &BankMap {
        &self.banks
    }
}

type BranchMap = HashMap<String, Branch>;
type BankMap = HashMap<String, Bank>;

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
    pub fn get_branch(&self, code: &str) -> Option<&Branch> {
        self.branches.get(code)
    }

    pub fn find_branches_by_name(&self, pattern: &str) -> Result<Vec<&Branch>, regex::Error> {
        let re = regex::Regex::new(pattern)?;
        let mut matched = vec![];
        for branch in self.branches.values() {
            if re.is_match(&branch.name) {
                matched.push(branch);
            }
        }
        Ok(matched)
    }

    pub fn find_branches_by_hira(&self, pattern: &str) -> Result<Vec<&Branch>, regex::Error> {
        let re = regex::Regex::new(pattern)?;
        let mut matched = vec![];
        for branch in self.branches.values() {
            if re.is_match(&branch.hira) {
                matched.push(branch);
            }
        }
        Ok(matched)
    }

    pub fn find_branches_by_kana(&self, pattern: &str) -> Result<Vec<&Branch>, regex::Error> {
        let re = regex::Regex::new(pattern)?;
        let mut matched = vec![];
        for branch in self.branches.values() {
            if re.is_match(&branch.kana) {
                matched.push(branch);
            }
        }
        Ok(matched)
    }

    pub fn find_branches_by_roma(&self, pattern: &str) -> Result<Vec<&Branch>, regex::Error> {
        let re = regex::Regex::new(pattern)?;
        let mut matched = vec![];
        for branch in self.branches.values() {
            if re.is_match(&branch.roma) {
                matched.push(branch);
            }
        }
        Ok(matched)
    }

    pub fn all_branches(&self) -> &BranchMap {
        &self.branches
    }
}

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
    let json_data = std::fs::read_to_string(file_path)?;
    let banks = parse_banks(&json_data).unwrap();
    Ok(banks)
}

fn load_branches_from_file(file_path: &str) -> std::result::Result<BranchMap, Box<dyn Error>> {
    let json_data = std::fs::read_to_string(file_path)?;
    parse_branches(&json_data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_banks() {
        let json_data = r#"{
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
        }"#;

        let banks = parse_banks(json_data).unwrap();
        assert_eq!(banks["0001"].name, "みずほ");
    }

    #[test]
    fn test_parse_branches() {
        let json_data = r#"{
            "001":{
              "code":"001",
              "name":"東京営業部",
              "kana":"トウキヨウ",
              "hira":"とうきよう",
              "roma":"toukiyou"
            },
            "004":{
              "code":"004",
              "name":"丸の内中央",
              "kana":"マルノウチチユウオウ",
              "hira":"まるのうちちゆうおう",
              "roma":"marunouchichiyuuou"
            }
        }"#;

        let branches = parse_branches(json_data).unwrap();
        assert_eq!(branches["001"].name, "東京営業部");
    }

    #[test]
    fn test_zengin_new() {
        let zengin = Zengin::new();
        assert_eq!(zengin.banks["0001"].name, "みずほ");
    }
}
