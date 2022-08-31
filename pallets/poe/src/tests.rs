use crate::{mock::*, Error};
use frame_support::{assert_noop, assert_ok, BoundedVec};
use super::*;

#[test]
fn create_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![1, 2];
        assert_ok!(PoeModule::create_claim(Origin::signed(1), claim.clone()));

        let bounded_claim = BoundedVec::<u8,<Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
        assert_eq!(Proofs::<Test>::get(&bounded_claim),
                   Some((1,frame_system::Pallet::<Test>::block_number()))
        );
    });
}

#[test]
fn create_claim_failed_when_claim_already_exists() {
    new_test_ext().execute_with(|| {
        let claim = vec![1, 2];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());

        assert_noop!(PoeModule::create_claim(Origin::signed(1), claim.clone()), Error::<Test>::ProofAlreadyExist);
    });
}

#[test]
fn revoke_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![1, 2];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        let bounded_claim = BoundedVec::<u8,<Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
        assert_ok!(PoeModule::revoke_claim(Origin::signed(1), claim.clone()));
        assert_eq!(Proofs::<Test>::get(&bounded_claim), None);
    });
}

#[test]
fn transfer_claim_works() {
    new_test_ext().execute_with(|| {
        let claim = vec![1, 2];
        let _ = PoeModule::create_claim(Origin::signed(1), claim.clone());
        let bounded_claim = BoundedVec::<u8,<Test as Config>::MaxClaimLength>::try_from(claim.clone()).unwrap();
        assert_ok!(PoeModule::transfer_claim(Origin::signed(1), claim.clone(), 2u64));
        assert_eq!(Proofs::<Test>::get(&bounded_claim), Some((2u64,frame_system::Pallet::<Test>::block_number())));
    });
}

//题目1.
//创建存证的测试用例
//撤销存证的测试用例
//转移存证的测试用例

//题目2.创建存证时，为存证内容的哈希值Vec<u8>
//设置长度上线，超过限制时候返回错误
//编写测试用例