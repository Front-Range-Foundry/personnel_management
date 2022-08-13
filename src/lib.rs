
pub mod personnel {
    use uuid::Uuid;
    pub struct PersonnelMember {
        pub name: String,
        pub id: Uuid,
    }

    impl PersonnelMember {
        pub fn new(name: String) -> PersonnelMember {
            PersonnelMember {
                name,
                id: Uuid::new_v4(),
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use uuid::Uuid;

    #[test]
    fn we_can_create_a_personnel_member() {
        let personnel_member = personnel::PersonnelMember {
            name: String::from("John Hammond"),
            id: Uuid::new_v4(),
        };

        assert_eq!(personnel_member.name, String::from("John Hammond"));
    }

    #[test]
    fn the_personnel_member_has_a_constructor() {
        let personnel_member = personnel::PersonnelMember::new(String::from("John Hammond"));
        assert_eq!(personnel_member.name, String::from("John Hammond"));
    }

    // #[test]
    // employee title enums lib 
}
