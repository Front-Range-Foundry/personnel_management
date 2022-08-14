use crate::department;
use uuid::Uuid;

pub struct PersonnelMember {
    pub name: String,
    pub id: Uuid,
    pub department_name: department::DepartmentName,
}

impl PersonnelMember {
    pub fn new(name: String, department_name: String) -> PersonnelMember {
        PersonnelMember {
            name,
            id: Uuid::new_v4(),
            department_name: department::utilities::string_to_department_type(department_name),
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn we_can_create_a_personnel_member() {
        let personnel_member = PersonnelMember {
            name: String::from("John Hammond"),
            id: Uuid::new_v4(),
            department_name: department::utilities::string_to_department_type(String::from("GENLAB")),
        };

        assert_eq!(personnel_member.name, String::from("John Hammond"));
    }

    #[test]
    fn the_personnel_member_has_a_constructor() {
        let personnel_member = PersonnelMember::new(
          String::from("John Hammond"), 
          String::from("GENLAB"));
        assert_eq!(personnel_member.name, String::from("John Hammond"));
    }

    #[test]
    fn the_personnel_member_is_entered_in_the_correct_department() {
        let personnel_member = PersonnelMember::new(
          String::from("John Hammond"), 
          String::from("GENLAB"));
          
        assert_eq!(personnel_member.department_name, department::DepartmentName::GENLAB);
    }
}
