/// This struct is used to manage user operations once user has authenticated.
/// Permissions are loaded throughout a request lifecycle.
/// That means, for every new request, permissions are loaded from DB again
pub struct AppUser {
    pub username: String,
    pub first_name: String,
    pub last_name: String,
    pub permissions: Vec<String>,
    pub email: String,
    pub is_admin: bool,
    pub groups: Vec<String>,
    pub is_authenticated: bool,
    pub is_active: bool
}

impl AppUser {
    pub fn is_authenticated(&self) -> bool {
        self.is_authenticated
    }

    pub fn is_active(&self) -> bool {
        self.is_active
    }

    pub fn is_admin(&self) -> bool {
        self.is_admin
    }

    pub fn has_permission(&self, permission_key: String) -> bool {
        self.is_admin || self.permissions.contains(&permission_key)
    }

    pub fn check_group_membership(&self, group_name: String) -> bool {
        self.groups.contains(&group_name)
    }
    
    pub fn get_all_groups(&self) -> Vec<String> {
        return self.groups.clone()
    }
}
