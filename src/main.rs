// src/main.rs

use tokio::net::TcpListener;
use tokio_tungstenite::{tungstenite::protocol::Message, WebSocketStream};
#[cfg(feature = "with-ssl")]
use tokio_native_tls::{TlsAcceptor, native_tls::Identity}; // Utilisation de TlsAcceptor pour TLS
#[cfg(not(feature = "with-ssl"))]
use tokio_tungstenite::accept_async;
use log::{info, warn, error};
use env_logger::Builder;
use colored::*;
use dotenv::dotenv;
use std::env;
#[cfg(feature = "with-ssl")]
use std::fs;
use futures_util::{SinkExt, StreamExt};
use std::io::Write;

// Fonction d'initialisation des logs colorés
fn init_logging() {
    Builder::new()
        .format(|buf, record| {
            let level = match record.level() {
                log::Level::Info => "INFO".blue(),
                log::Level::Warn => "WARN".yellow(),
                log::Level::Error => "ERROR".red(),
                log::Level::Debug => "DEBUG".green(),
                _ => "LOG".normal(),
            };
            writeln!(buf, "[{}] - {}", level, record.args()).map_err(|e| error!("Log error: {:?}", e)).ok();
            Ok(())
        })
        .filter(None, log::LevelFilter::Debug)
        .init();
}

// Fonction pour gérer chaque client connecté
async fn handle_client<S>(ws_stream: WebSocketStream<S>)
where
    S: tokio::io::AsyncRead + tokio::io::AsyncWrite + Unpin + Send + 'static,
{
    let (mut write, mut read) = ws_stream.split();

    info!("Connexion WebSocket réussie, en attente de messages du client.");

    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(text)) => {
                info!("Message reçu : {}", text);
                let response = format!("Message reçu : {}", text);
                write.send(Message::Text(response)).await.unwrap();
            },
            Ok(Message::Close(_)) => {
                info!("Client déconnecté !");
                return;
            },
            Err(e) => {
                warn!("Erreur avec le client : {:?}", e);
                return;
            },
            _ => {}
        }
    }
}

// Point d'entrée principal du serveur
#[tokio::main]
async fn main() {
    dotenv().ok();
    env::set_var("RUST_LOG", "debug");
    init_logging();

    let mode = env::var("MODE").unwrap_or_else(|_| "dev".to_string());
    let addr = if mode == "prod" { "0.0.0.0:443" } else { "127.0.0.1:8080" };

    let listener = TcpListener::bind(addr).await.expect("Impossible de démarrer le serveur");
    info!("Kronos-Server démarré en mode {} sur l'adresse {}", mode, addr);
    info!("En attente de connexions clients...");

    while let Ok((stream, _)) = listener.accept().await {
        let mode_clone = mode.clone(); // Crée une copie de `mode` pour chaque tâche
    
        tokio::spawn(async move {
            if mode_clone == "prod" {
                // Configuration pour le mode production avec TLS
                #[cfg(feature = "with-ssl")]
                {
                    let cert_data = fs::read("path/to/your/cert.pfx").expect("Impossible de lire le fichier de certificat");
                    let identity = Identity::from_pkcs12(&cert_data, "password").expect("Erreur de chargement de l'identité TLS");
                    let acceptor = TlsAcceptor::from(native_tls::TlsAcceptor::new(identity).unwrap());
                    let tls_stream = acceptor.accept(stream).await.expect("Erreur de connexion TLS");
                    let ws_stream = tokio_tungstenite::accept_async(tls_stream)
                        .await
                        .expect("Erreur de connexion WebSocket TLS");
                    handle_client(ws_stream).await;
                }
            } else {
                // Mode développement, sans TLS
                #[cfg(not(feature = "with-ssl"))]
                {
                    let ws_stream = accept_async(stream)
                        .await
                        .expect("Erreur de connexion WebSocket");
                    handle_client(ws_stream).await;
                }
            }
        });
    }
}