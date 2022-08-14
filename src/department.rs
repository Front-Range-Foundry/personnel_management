
use crate::personnel::PersonnelMember;

#[derive(PartialEq)]
#[derive(Debug)]
pub enum DepartmentName {
  GENLAB,
  SECURITY,
  TECHNICAL,
  CUSTODIAL,
  ADMINISTRATIVE,
  SUPPORT,
  ASSETMANAGEMENT, 
  VETERINARY,
  MEDICAL,
  BOTANICAL,
  ACCOUNTING,
  LAW,
  OTHER
}

pub struct Department {
  pub id: DepartmentName,
  pub personnel: Vec<PersonnelMember>,
}

impl Department {
  pub fn new(id: DepartmentName) -> Department {
    Department {
      id,
      personnel: Vec::new(),
    }
  }
}

pub mod admin_tools {
  use uuid::Uuid;
  use super::*;
  use utilities;

  pub fn create_department(department_type: String) -> Department {
    let department_type_enum = utilities::string_to_department_type(department_type);
    Department::new(department_type_enum)
  }

  pub fn add_personnel_member(department: &mut Department, personnel_member: PersonnelMember) {
    department.personnel.push(personnel_member);
  }

  pub fn remove_personnel_member(department: &mut Department, id: Uuid) {
    department.personnel.retain(|p| &p.id != &id);
  }
}

pub mod utilities {
  use super::DepartmentName;

  pub fn string_to_department_type(val: String) -> DepartmentName {
    match val.as_str() {
      "GENLAB" => DepartmentName::GENLAB,
      "SECURITY" => DepartmentName::SECURITY,
      "TECHNICAL" => DepartmentName::TECHNICAL,
      "CUSTODIAL" => DepartmentName::CUSTODIAL,
      "ADMINISTRATIVE" => DepartmentName::ADMINISTRATIVE,
      "SUPPORT" => DepartmentName::SUPPORT,
      "ASSETMANAGEMENT" => DepartmentName::ASSETMANAGEMENT,
      "VETERINARY" => DepartmentName::VETERINARY,
      "MEDICAL" => DepartmentName::MEDICAL,
      "BOTANICAL" => DepartmentName::BOTANICAL,
      "ACCOUNTING" => DepartmentName::ACCOUNTING,
      "LAW" => DepartmentName::LAW,
      _ => DepartmentName::OTHER,
    }
  }
}

#[cfg(test)]
pub mod tests {
  use uuid::Uuid;
  use super::*;

  #[test]
  fn we_can_create_a_department() {
    let department = admin_tools::create_department(String::from("GENLAB"));
    assert_eq!(department.id, DepartmentName::GENLAB);
  }

  #[test]
  fn we_can_add_a_personnel_member_to_a_department() {
    let mut department = Department::new(DepartmentName::GENLAB);
    let personnel_member = PersonnelMember::new(String::from("John Hammond"), String::from("GENLAB"));
    admin_tools::add_personnel_member(&mut department, personnel_member);
    assert_eq!(department.personnel.len(), 1);
  }

  #[test]
  fn we_can_remove_a_personnel_member_from_a_department() {
    let mut department = Department::new(DepartmentName::GENLAB);
    let personnel_member = PersonnelMember::new(String::from("John Hammond"), String::from("GENLAB"));
    let id = String::from(&personnel_member.id.to_string());
    admin_tools::add_personnel_member(&mut department, personnel_member);
    admin_tools::remove_personnel_member(&mut department, Uuid::parse_str(&id).unwrap());
    assert_eq!(department.personnel.len(), 0);
  }

  #[test]
  fn utilities_returns_the_correct_department_type() {
    let department_type = utilities::string_to_department_type(String::from("GENLAB"));
    assert_eq!(department_type, DepartmentName::GENLAB);
  }
}