mod cangjie;
mod config;
mod lsp_server;
mod proxy;
mod util;

use cangjie::CangjieExtension;
use zed_extension_api::register_extension;

register_extension!(CangjieExtension);
