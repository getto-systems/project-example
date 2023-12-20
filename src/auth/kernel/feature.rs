use crate::auth::ticket::{
    authorize::data::{AuthorizeSuccess, CheckAuthorizeTokenSuccess},
    kernel::data::AuthPermissionRequired,
};

pub trait AsCheckedInfra<M> {
    fn required(&self) -> AuthPermissionRequired;
    fn as_authorized_infra(&self, _: &CheckAuthorizeTokenSuccess) -> M;
}

pub trait AsAuthorizedInfra<M> {
    fn required(&self) -> AuthPermissionRequired;
    fn as_authorized_infra(&self, _: &AuthorizeSuccess) -> M;
}

impl<F: AsAuthorizedInfra<M1> + AsAuthorizedInfra<M2>, M1, M2> AsAuthorizedInfra<(M1, M2)> for F {
    fn required(&self) -> AuthPermissionRequired {
        AsAuthorizedInfra::<M1>::required(self).union(AsAuthorizedInfra::<M2>::required(self))
    }
    fn as_authorized_infra(&self, auth: &AuthorizeSuccess) -> (M1, M2) {
        (
            AsAuthorizedInfra::<M1>::as_authorized_infra(self, auth),
            AsAuthorizedInfra::<M2>::as_authorized_infra(self, auth),
        )
    }
}

impl<F: AsAuthorizedInfra<M1> + AsAuthorizedInfra<M2> + AsAuthorizedInfra<M3>, M1, M2, M3>
    AsAuthorizedInfra<(M1, M2, M3)> for F
{
    fn required(&self) -> AuthPermissionRequired {
        AsAuthorizedInfra::<M1>::required(self)
            .union(AsAuthorizedInfra::<M2>::required(self))
            .union(AsAuthorizedInfra::<M3>::required(self))
    }
    fn as_authorized_infra(&self, auth: &AuthorizeSuccess) -> (M1, M2, M3) {
        (
            AsAuthorizedInfra::<M1>::as_authorized_infra(self, auth),
            AsAuthorizedInfra::<M2>::as_authorized_infra(self, auth),
            AsAuthorizedInfra::<M3>::as_authorized_infra(self, auth),
        )
    }
}
