use among_us_server::game::Meeting;
use uuid::Uuid;

#[test]
fn meeting_finds_user() {
    let user_1 = Uuid::from_u128(1);
    let user_2 = Uuid::from_u128(2);
    let user_3 = Uuid::from_u128(3);
    let mut meeting = Meeting::new(3);

    meeting.add_vote(user_1, user_3);
    meeting.add_vote(user_2, user_3);
    meeting.add_vote(user_3, user_1);
    let voted_out_user = meeting.person_voted_out();

    assert_eq!(voted_out_user, Some(user_3));
}

#[test]
fn meeting_is_inconclusive() {
    let user_1 = Uuid::from_u128(1);
    let user_2 = Uuid::from_u128(2);
    let user_3 = Uuid::from_u128(3);
    let mut meeting = Meeting::new(3);

    meeting.add_vote(user_1, user_3);
    meeting.add_vote(user_2, user_1);
    meeting.add_vote(user_3, user_2);
    let voted_out_user = meeting.person_voted_out();

    assert_eq!(voted_out_user, None);
}
