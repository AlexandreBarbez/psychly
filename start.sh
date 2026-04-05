#!/bin/bash
# Psychly — Portable launcher script
# Starts Ollama with portable paths, waits for readiness, then launches the app.

set -e

# Resolve the directory where this script lives (application root)
APP_ROOT="$(cd "$(dirname "$0")" && pwd)"

# Portable directory layout
export OLLAMA_MODELS="$APP_ROOT/models"
export OLLAMA_HOST="http://127.0.0.1:11434"
DATA_DIR="$APP_ROOT/data"
OLLAMA_BIN="$APP_ROOT/ollama/ollama"
APP_BIN="$APP_ROOT/app/Psychly.app/Contents/MacOS/Psychly"

# Ensure required directories exist
mkdir -p "$DATA_DIR" "$OLLAMA_MODELS"

# --- Start Ollama ---
OLLAMA_PID=""

cleanup() {
    echo ""
    echo "[Psychly] Arrêt en cours..."
    if [ -n "$OLLAMA_PID" ] && kill -0 "$OLLAMA_PID" 2>/dev/null; then
        kill "$OLLAMA_PID" 2>/dev/null
        wait "$OLLAMA_PID" 2>/dev/null || true
        echo "[Psychly] Ollama arrêté."
    fi
    exit 0
}

trap cleanup EXIT INT TERM

echo "[Psychly] Démarrage d'Ollama..."

if [ -x "$OLLAMA_BIN" ]; then
    "$OLLAMA_BIN" serve &
    OLLAMA_PID=$!
elif command -v ollama &>/dev/null; then
    ollama serve &
    OLLAMA_PID=$!
else
    echo "[Psychly] Erreur : Ollama introuvable."
    echo "  Placez le binaire Ollama dans : $OLLAMA_BIN"
    echo "  Ou installez Ollama : https://ollama.com"
    exit 1
fi

# Wait for Ollama to be ready (up to 30 seconds)
echo "[Psychly] Attente de la disponibilité d'Ollama..."
MAX_WAIT=30
WAITED=0
while [ $WAITED -lt $MAX_WAIT ]; do
    if curl -sf "$OLLAMA_HOST/api/tags" > /dev/null 2>&1; then
        echo "[Psychly] Ollama est prêt."
        break
    fi
    sleep 1
    WAITED=$((WAITED + 1))
done

if [ $WAITED -ge $MAX_WAIT ]; then
    echo "[Psychly] Attention : Ollama n'a pas répondu dans les $MAX_WAIT secondes."
    echo "  Le chat et l'analyse pourraient ne pas fonctionner."
fi

# --- Launch the application ---
echo "[Psychly] Lancement de l'application..."

if [ -x "$APP_BIN" ]; then
    "$APP_BIN"
elif [ -d "$APP_ROOT/app/Psychly.app" ]; then
    open "$APP_ROOT/app/Psychly.app" --wait-apps
else
    echo "[Psychly] Application introuvable."
    echo "  Compilez l'application (à la racine) avec : npm run tauri build"
    echo "  Puis copiez le .app généré (dans src-tauri/target/release/bundle/macos/) vers : $APP_ROOT/app/"
    exit 1
fi

echo "[Psychly] Application fermée."
