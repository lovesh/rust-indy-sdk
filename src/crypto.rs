use super::{ErrorCode, IndyHandle};

use std::ffi::CString;

use ffi::crypto;

use utils::results::ResultHandler;
use utils::callbacks::ClosureHandler;

pub struct Key {}

impl Key {
    pub fn create(wallet_handle: IndyHandle, my_key_json: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let my_key_json = c_str!(my_key_json);

        let err = unsafe {
            crypto::indy_create_key(command_handle, wallet_handle, my_key_json.as_ptr(), cb)
        };
        ResultHandler::one(err, receiver)
    }

    pub fn set_metadata(wallet_handle: IndyHandle, verkey: &str) -> Result<(), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec();

        let verkey = c_str!(verkey);

        let err = unsafe {
            crypto::indy_set_key_metadata(command_handle, wallet_handle, verkey.as_ptr(), cb)
        };
        ResultHandler::empty(err, receiver)
    }

    pub fn get_metadata(wallet_handle: IndyHandle, verkey: &str) -> Result<String, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string();

        let verkey = c_str!(verkey);

        let err = unsafe {
            crypto::indy_get_key_metadata(command_handle, wallet_handle, verkey.as_ptr(), cb)
        };

        ResultHandler::one(err, receiver)
    }
}

pub struct Crypto {}

impl Crypto {
    pub fn sign(wallet_handle: IndyHandle, signer_vk: &str, message: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_u8();

        let signer_vk = c_str!(signer_vk);
        let err = unsafe {
            crypto::indy_crypto_sign(command_handle, wallet_handle, signer_vk.as_ptr(),
                             message.as_ptr() as *const u8,
                             message.len() as u32,
                             cb)
        };

        ResultHandler::one(err, receiver)
    }

    pub fn verify(wallet_handle: IndyHandle, message: &[u8], signature: &[u8]) -> Result<bool, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_bool();

        let err = unsafe {
            crypto::indy_crypto_verify(command_handle, wallet_handle,
                               message.as_ptr() as *const u8, message.len() as u32,
                               signature.as_ptr() as *const u8, signature.len() as u32, cb)
        };

        ResultHandler::one(err, receiver)
    }

    pub fn auth_crypt(wallet_handle: IndyHandle, sender_vk: &str, recipient_vk: &str, message: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_u8();

        let sender_vk = c_str!(sender_vk);
        let recipient_vk = c_str!(recipient_vk);
        let err = unsafe {
            crypto::indy_crypto_auth_crypt(command_handle, wallet_handle,
                                   sender_vk.as_ptr(),
                                    recipient_vk.as_ptr(),
                                    message.as_ptr() as *const u8,
                                    message.len() as u32, cb)
        };
        ResultHandler::one(err, receiver)
    }

    pub fn auth_decrypt(wallet_handle: IndyHandle, recipient_vk: &str, encrypted_message: &[u8]) -> Result<(String, Vec<u8>), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_u8();

        let recipient_vk = c_str!(recipient_vk);
        let err = unsafe {
            crypto::indy_crypto_auth_decrypt(command_handle,
                                     wallet_handle,
                                     recipient_vk.as_ptr(),
                                     encrypted_message.as_ptr() as *const u8,
                                     encrypted_message.len() as u32, cb)
        };

        ResultHandler::two(err, receiver)
    }

    pub fn anon_crypt(wallet_handle: IndyHandle, recipient_vk: &str, message: &[u8]) -> Result<Vec<u8>, ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_u8();

        let recipient_vk = c_str!(recipient_vk);
        let err = unsafe {
            crypto::indy_crypto_anon_crypt(command_handle,
                                   wallet_handle,
                                   recipient_vk.as_ptr(),
                                   message.as_ptr() as *const u8,
                                    message.len() as u32,
                                    cb)
        };

        ResultHandler::one(err, receiver)
    }

    pub fn anon_decrypt(wallet_handle: IndyHandle, recipient_vk: &str, encrypted_message: &[u8]) -> Result<(String, Vec<u8>), ErrorCode> {
        let (receiver, command_handle, cb) = ClosureHandler::cb_ec_string_u8();

        let recipient_vk = c_str!(recipient_vk);
        let err = unsafe {
            crypto::indy_crypto_anon_decrypt(command_handle,
                                     wallet_handle,
                                     recipient_vk.as_ptr(),
                                     encrypted_message.as_ptr() as *const u8,
                                     encrypted_message.len() as u32, cb)
        };

        ResultHandler::two(err, receiver)
    }
}