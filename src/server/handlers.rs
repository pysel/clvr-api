use std::{str::FromStr, sync::{Arc, Mutex}};
use actix_web::{get, post, web, HttpResponse, Responder};
use alloy::primitives::{Address, FixedBytes, PrimitiveSignature, U256};
use log::{info, warn};
use crate::server::handlers_types::*;
use crate::server::{eip2612::verify_eip2612_signature};

pub type ScheduledDatabase = Arc<Mutex<Vec<ScheduledTrade>>>;

const LOG_TARGET: &str = "server::handlers";

#[get("/num_trades")]
pub async fn num_trades(db: web::Data<ScheduledDatabase>) -> impl Responder {
    info!(target: LOG_TARGET, "num_trades called");
    HttpResponse::Ok().json(NumTradesResponse {
        num_trades: db.lock().unwrap().len() as u64,
    })
}

/*
MOCK REQUEST BODY:
{
    "from": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
    "swap_params": {
        "token_in": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
        "token_out": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
        "fee": 1000,
        "recipient": "0xf39fd6e51aad88f6f4ce6ab8827279cfffb92266",
        "deadline": 1000,
        "amount_in": 1000,
        "amount_out_minimum": 1000,
        "sqrt_price_limit_x96": 1000
    },
    "permit_msg": "4f72bf4ece92162febe06cd70061da75707eb457f20c2a8ce580d424d5195049",
    "signature": "14e37d06070dca6bd1c14087f2857672c7bc385a5a09366de67c591b26a0e929442dbb42f8a66133aaff860a1f5afbbfb79b4a808ca3b7f662f90d7e68a265251b"
}
 */
#[post("/submit_trade")]
pub async fn submit_trade(trade_request: web::Json<ScheduleRequest>, db: web::Data<ScheduledDatabase>,) -> impl Responder {
    info!(target: LOG_TARGET, "submit_trade called");

    let mut db = db.lock().unwrap();

    // verify from address
    let from = Address::from_str(&trade_request.from).unwrap_or(Address::ZERO);
    if from == Address::ZERO {
        warn!(target: LOG_TARGET, "Invalid from address");
        return HttpResponse::BadRequest().json(ScheduleResponse {
            success: false,
            message: "Invalid from address".to_string(),
        });
    }
    
    // verify signature (return default types except panicking so that verification fails gracefully)
    let permit_message: FixedBytes<32> = FixedBytes::from_str(&trade_request.permit_msg)
        .unwrap_or(FixedBytes::ZERO);
    let signature: PrimitiveSignature = PrimitiveSignature::from_str(&trade_request.signature)
        .unwrap_or(PrimitiveSignature::new(U256::ZERO, U256::ZERO, false));
    let signer: Address = Address::from_str(&trade_request.from)
        .unwrap_or(Address::ZERO);
    
    if !verify_eip2612_signature(permit_message, signature, signer) {
        warn!(target: LOG_TARGET, "Invalid signature, message or signer");
        return HttpResponse::BadRequest().json(ScheduleResponse {
            success: false,
            message: "Invalid signature, message or signer".to_string(),
        });
    }
    
    let scheduled_trade: ScheduledTrade = trade_request.into_inner().into();
    let scheduled_trade_clone = scheduled_trade.clone();
    db.push(scheduled_trade);

    HttpResponse::Created().json(ScheduleResponse {
        success: true,
        message: scheduled_trade_clone.to_string(),
    })
}
