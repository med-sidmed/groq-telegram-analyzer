#!/bin/bash

# Script de déploiement automatisé pour Telegram AI Analyzer

echo "🚀 Démarrage du déploiement Docker..."

# Vérifier la présence du fichier .env
if [ ! -f .env ]; then
    echo "❌ Erreur : Le fichier .env est manquant !"
    exit 1
fi

# Tirer les dernières modifications (optionnel si utilisé dans un workflow CI/CD)
# git pull origin master

echo "📦 Construction de l'image (cela peut prendre quelques minutes)..."
docker compose build

echo "🔄 Redémarrage des services..."
docker compose up -d

echo "✅ Déploiement terminé !"
echo "📊 Pour voir les logs : docker compose logs -f"
