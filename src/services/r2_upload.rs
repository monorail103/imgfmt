// Cloudflare R2に、変換した画像をアップロードする
use worker::*;

use std::env;
use reqwest;
use crate::types::{ConversionParams};

