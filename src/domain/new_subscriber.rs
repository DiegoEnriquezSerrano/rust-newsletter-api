use crate::domain::SubscriberEmail;
use crate::domain::SubscriberName;
use uuid::Uuid;

pub struct NewSubscriber {
    // We are not using `String` anymore!
    pub email: SubscriberEmail,
    pub name: SubscriberName,
    pub user_id: Uuid,
}
