use super::*;

#[test]
fn begin_turn_if_idle() {
    let mut session = Session::new();
    let user_input = "Hello, how are you?".to_string();

    let turn_result = session.begin_turn(user_input.clone());

    assert!(turn_result.is_ok());
    let turn = turn_result.unwrap();
    assert!(!turn.request_id.is_empty());

    match &session.state {
        State::Generating {
            request_id,
            user_input: input,
        } => {
            assert_eq!(input.as_ref().unwrap(), &user_input);
            assert_eq!(request_id, &turn.request_id);
        }
        _ => panic!("Expected state to be Generating"),
    }
}

#[test]
fn begin_turn_if_generating() {
    let mut session = Session::new();
    let user_input1 = "Hello, how are you?".to_string();
    let user_input2 = "What's the weather like?".to_string();

    let result1 = session.begin_turn(user_input1.clone());
    assert!(result1.is_ok());

    let result2 = session.begin_turn(user_input2.clone());
    assert!(result2.is_err());
}

#[test]
fn cancel_turn_if_generating() {
    let mut session = Session::new();
    let user_input = "Hello, how are you?".to_string();

    let result = session.begin_turn(user_input.clone());
    assert!(result.is_ok());

    let cancel_result = session.cancel_turn(result.unwrap().request_id.clone());
    assert!(cancel_result.is_ok());

    match &session.state {
        State::Idle => {}
        _ => panic!("Expected state to be Idle"),
    }
}

#[test]
fn failed_turn_if_generating() {
    let mut session = Session::new();
    let user_input = "Hello, how are you?".to_string();

    let result = session.begin_turn(user_input.clone());
    assert!(result.is_ok());

    let fail_result = session.fail_turn(result.unwrap().request_id.clone());
    assert!(fail_result.is_ok());

    match &session.state {
        State::Idle => {}
        _ => panic!("Expected state to be Idle"),
    }
}
