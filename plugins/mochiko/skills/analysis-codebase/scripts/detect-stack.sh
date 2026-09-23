#!/bin/bash
#
# detect-stack.sh - Detect technology stack from a codebase
#
# Usage:
#     bash detect-stack.sh <path-to-project>
#
# Output:
#     JSON with detected technologies
#
# This script provides fast, deterministic detection of:
# - Project type (nodejs, python, go, rust, java, ruby, flutter)
# - Package manager (npm, yarn, pnpm, pip, poetry, cargo, etc.)
# - Web frameworks (express, fastapi, django, flask, gin, etc.)
# - ORM/Database (prisma, typeorm, sqlalchemy, mongoose, etc.)
# - Architecture pattern (layered, feature-based, mvc, clean)
# - CI/CD (github-actions, gitlab-ci, jenkins, etc.)
# - Design system (UI framework, CSS system, fonts, token files, component library)

set -e

PROJECT_DIR="${1:-.}"

if [[ ! -d "$PROJECT_DIR" ]]; then
    echo '{"error": "Directory not found: '"$PROJECT_DIR"'"}'
    exit 1
fi

cd "$PROJECT_DIR"

# Initialize detection arrays
PROJECT_TYPE=""
PACKAGE_MANAGER=""
FRAMEWORKS=""
ORMS=""
ARCHITECTURE=""
CI_CD=""

# ============================================================================
# Project Type Detection
# ============================================================================

detect_project_type() {
    if [[ -f "package.json" ]]; then
        PROJECT_TYPE="nodejs"
        if [[ -f "yarn.lock" ]]; then
            PACKAGE_MANAGER="yarn"
        elif [[ -f "pnpm-lock.yaml" ]]; then
            PACKAGE_MANAGER="pnpm"
        elif [[ -f "bun.lockb" ]]; then
            PACKAGE_MANAGER="bun"
        else
            PACKAGE_MANAGER="npm"
        fi
    elif [[ -f "pyproject.toml" ]]; then
        PROJECT_TYPE="python"
        if grep -q "poetry" pyproject.toml 2>/dev/null; then
            PACKAGE_MANAGER="poetry"
        elif grep -q "pdm" pyproject.toml 2>/dev/null; then
            PACKAGE_MANAGER="pdm"
        else
            PACKAGE_MANAGER="pip"
        fi
    elif [[ -f "requirements.txt" ]] || [[ -f "setup.py" ]]; then
        PROJECT_TYPE="python"
        PACKAGE_MANAGER="pip"
    elif [[ -f "go.mod" ]]; then
        PROJECT_TYPE="go"
        PACKAGE_MANAGER="go-modules"
    elif [[ -f "Cargo.toml" ]]; then
        PROJECT_TYPE="rust"
        PACKAGE_MANAGER="cargo"
    elif [[ -f "pom.xml" ]]; then
        PROJECT_TYPE="java"
        PACKAGE_MANAGER="maven"
    elif [[ -f "build.gradle" ]] || [[ -f "build.gradle.kts" ]]; then
        PROJECT_TYPE="java"
        PACKAGE_MANAGER="gradle"
    elif [[ -f "Gemfile" ]]; then
        PROJECT_TYPE="ruby"
        PACKAGE_MANAGER="bundler"
    elif [[ -f "pubspec.yaml" ]]; then
        PROJECT_TYPE="flutter"
        PACKAGE_MANAGER="pub"
    elif [[ -f "mix.exs" ]]; then
        PROJECT_TYPE="elixir"
        PACKAGE_MANAGER="mix"
    else
        PROJECT_TYPE="unknown"
        PACKAGE_MANAGER="unknown"
    fi
}

# ============================================================================
# Framework Detection
# ============================================================================

detect_frameworks() {
    local frameworks=()

    # Node.js frameworks
    if [[ -f "package.json" ]]; then
        local pkg_content
        pkg_content=$(cat package.json 2>/dev/null || echo "{}")

        # Express
        if echo "$pkg_content" | grep -q '"express"'; then
            frameworks+=("express")
        fi
        # Fastify
        if echo "$pkg_content" | grep -q '"fastify"'; then
            frameworks+=("fastify")
        fi
        # NestJS
        if echo "$pkg_content" | grep -q '"@nestjs/core"'; then
            frameworks+=("nestjs")
        fi
        # Next.js
        if echo "$pkg_content" | grep -q '"next"'; then
            frameworks+=("nextjs")
        fi
        # Hono
        if echo "$pkg_content" | grep -q '"hono"'; then
            frameworks+=("hono")
        fi
        # Koa
        if echo "$pkg_content" | grep -q '"koa"'; then
            frameworks+=("koa")
        fi
    fi

    # Python frameworks
    if [[ "$PROJECT_TYPE" == "python" ]]; then
        local py_deps=""
        [[ -f "requirements.txt" ]] && py_deps+=$(cat requirements.txt 2>/dev/null)
        [[ -f "pyproject.toml" ]] && py_deps+=$(cat pyproject.toml 2>/dev/null)

        if echo "$py_deps" | grep -qi "fastapi"; then
            frameworks+=("fastapi")
        fi
        if echo "$py_deps" | grep -qi "django"; then
            frameworks+=("django")
        fi
        if echo "$py_deps" | grep -qi "flask"; then
            frameworks+=("flask")
        fi
        if echo "$py_deps" | grep -qi "starlette"; then
            frameworks+=("starlette")
        fi
    fi

    # Go frameworks
    if [[ -f "go.mod" ]]; then
        local go_mod
        go_mod=$(cat go.mod 2>/dev/null || echo "")

        if echo "$go_mod" | grep -q "gin-gonic/gin"; then
            frameworks+=("gin")
        fi
        if echo "$go_mod" | grep -q "labstack/echo"; then
            frameworks+=("echo")
        fi
        if echo "$go_mod" | grep -q "gofiber/fiber"; then
            frameworks+=("fiber")
        fi
    fi

    # Ruby frameworks
    if [[ -f "Gemfile" ]]; then
        if grep -q "rails" Gemfile 2>/dev/null; then
            frameworks+=("rails")
        fi
        if grep -q "sinatra" Gemfile 2>/dev/null; then
            frameworks+=("sinatra")
        fi
    fi

    # Java frameworks
    if [[ -f "pom.xml" ]] || [[ -f "build.gradle" ]] || [[ -f "build.gradle.kts" ]]; then
        local java_deps=""
        [[ -f "pom.xml" ]] && java_deps+=$(cat pom.xml 2>/dev/null)
        [[ -f "build.gradle" ]] && java_deps+=$(cat build.gradle 2>/dev/null)
        [[ -f "build.gradle.kts" ]] && java_deps+=$(cat build.gradle.kts 2>/dev/null)

        if echo "$java_deps" | grep -qi "spring-boot"; then
            frameworks+=("spring-boot")
        fi
    fi

    # Convert to JSON array
    if [[ ${#frameworks[@]} -eq 0 ]]; then
        FRAMEWORKS="[]"
    else
        FRAMEWORKS=$(printf '%s\n' "${frameworks[@]}" | jq -R . | jq -s .)
    fi
}

# ============================================================================
# ORM Detection
# ============================================================================

detect_orms() {
    local orms=()

    # Node.js ORMs
    if [[ -f "package.json" ]]; then
        local pkg_content
        pkg_content=$(cat package.json 2>/dev/null || echo "{}")

        if echo "$pkg_content" | grep -q '"@prisma/client"' || [[ -f "prisma/schema.prisma" ]] || [[ -f "schema.prisma" ]]; then
            orms+=("prisma")
        fi
        if echo "$pkg_content" | grep -q '"typeorm"'; then
            orms+=("typeorm")
        fi
        if echo "$pkg_content" | grep -q '"sequelize"'; then
            orms+=("sequelize")
        fi
        if echo "$pkg_content" | grep -q '"mongoose"'; then
            orms+=("mongoose")
        fi
        if echo "$pkg_content" | grep -q '"drizzle-orm"'; then
            orms+=("drizzle")
        fi
        if echo "$pkg_content" | grep -q '"kysely"'; then
            orms+=("kysely")
        fi
    fi

    # Python ORMs
    if [[ "$PROJECT_TYPE" == "python" ]]; then
        local py_deps=""
        [[ -f "requirements.txt" ]] && py_deps+=$(cat requirements.txt 2>/dev/null)
        [[ -f "pyproject.toml" ]] && py_deps+=$(cat pyproject.toml 2>/dev/null)

        if echo "$py_deps" | grep -qi "sqlalchemy"; then
            orms+=("sqlalchemy")
        fi
        if echo "$py_deps" | grep -qi "django"; then
            orms+=("django-orm")
        fi
        if echo "$py_deps" | grep -qi "tortoise-orm"; then
            orms+=("tortoise")
        fi
        if echo "$py_deps" | grep -qi "peewee"; then
            orms+=("peewee")
        fi
    fi

    # Go ORMs
    if [[ -f "go.mod" ]]; then
        local go_mod
        go_mod=$(cat go.mod 2>/dev/null || echo "")

        if echo "$go_mod" | grep -q "gorm.io/gorm"; then
            orms+=("gorm")
        fi
        if echo "$go_mod" | grep -q "ent/ent"; then
            orms+=("ent")
        fi
    fi

    # Ruby ORMs
    if [[ -f "Gemfile" ]]; then
        if grep -q "activerecord" Gemfile 2>/dev/null || grep -q "rails" Gemfile 2>/dev/null; then
            orms+=("activerecord")
        fi
    fi

    # Convert to JSON array
    if [[ ${#orms[@]} -eq 0 ]]; then
        ORMS="[]"
    else
        ORMS=$(printf '%s\n' "${orms[@]}" | jq -R . | jq -s .)
    fi
}

# ============================================================================
# Architecture Pattern Detection
# ============================================================================

detect_architecture() {
    local patterns=()

    # Check for common directory patterns
    if [[ -d "src/domain" ]] || [[ -d "src/application" ]] || [[ -d "src/infrastructure" ]]; then
        patterns+=("clean-architecture")
    fi

    if [[ -d "src/models" ]] && [[ -d "src/controllers" ]] && [[ -d "src/views" ]]; then
        patterns+=("mvc")
    elif [[ -d "app/models" ]] && [[ -d "app/controllers" ]] && [[ -d "app/views" ]]; then
        patterns+=("mvc")
    fi

    if [[ -d "src/services" ]] && [[ -d "src/repositories" ]]; then
        patterns+=("layered")
    elif [[ -d "src/models" ]] && [[ -d "src/services" ]] && [[ -d "src/controllers" ]]; then
        patterns+=("layered")
    fi

    # Feature-based detection (look for feature folders)
    local feature_dirs=0
    for dir in src/features src/modules src/auth src/users src/tasks src/api; do
        [[ -d "$dir" ]] && ((feature_dirs++))
    done
    if [[ $feature_dirs -ge 2 ]]; then
        patterns+=("feature-based")
    fi

    # Serverless
    if [[ -f "serverless.yml" ]] || [[ -f "serverless.yaml" ]] || [[ -d "functions" ]] || [[ -d "lambda" ]]; then
        patterns+=("serverless")
    fi

    # Microservices
    if [[ -f "docker-compose.yml" ]] || [[ -f "docker-compose.yaml" ]]; then
        # Check if it defines multiple services
        local service_count
        service_count=$(grep -c "^\s*[a-zA-Z_-]*:$" docker-compose.yml 2>/dev/null || echo "0")
        if [[ $service_count -gt 2 ]]; then
            patterns+=("microservices")
        fi
    fi

    # Monorepo
    if [[ -f "lerna.json" ]] || [[ -f "pnpm-workspace.yaml" ]] || [[ -d "packages" ]]; then
        patterns+=("monorepo")
    fi

    # Convert to JSON array
    if [[ ${#patterns[@]} -eq 0 ]]; then
        ARCHITECTURE="[]"
    else
        ARCHITECTURE=$(printf '%s\n' "${patterns[@]}" | jq -R . | jq -s .)
    fi
}

# ============================================================================
# CI/CD Detection
# ============================================================================

detect_cicd() {
    local cicd=()

    if [[ -d ".github/workflows" ]]; then
        cicd+=("github-actions")
    fi
    if [[ -f ".gitlab-ci.yml" ]]; then
        cicd+=("gitlab-ci")
    fi
    if [[ -f "Jenkinsfile" ]]; then
        cicd+=("jenkins")
    fi
    if [[ -f ".circleci/config.yml" ]]; then
        cicd+=("circleci")
    fi
    if [[ -f ".travis.yml" ]]; then
        cicd+=("travis")
    fi
    if [[ -f "azure-pipelines.yml" ]]; then
        cicd+=("azure-devops")
    fi
    if [[ -f "bitbucket-pipelines.yml" ]]; then
        cicd+=("bitbucket")
    fi

    # Convert to JSON array
    if [[ ${#cicd[@]} -eq 0 ]]; then
        CI_CD="[]"
    else
        CI_CD=$(printf '%s\n' "${cicd[@]}" | jq -R . | jq -s .)
    fi
}

# ============================================================================
# File Presence Detection
# ============================================================================

detect_files() {
    local files_json="{"
    local first=true

    # Key files to check
    local files_to_check=(
        "package.json"
        "tsconfig.json"
        "pyproject.toml"
        "requirements.txt"
        "go.mod"
        "Cargo.toml"
        "Gemfile"
        "Dockerfile"
        "docker-compose.yml"
        ".env.example"
        "README.md"
        "CLAUDE.md"
    )

    for file in "${files_to_check[@]}"; do
        if [[ "$first" == "true" ]]; then
            first=false
        else
            files_json+=","
        fi

        if [[ -f "$file" ]]; then
            files_json+="\"$file\":true"
        else
            files_json+="\"$file\":false"
        fi
    done

    # Check directories
    local dirs_to_check=(
        ".github/workflows"
        "src"
        "tests"
        "prisma"
    )

    for dir in "${dirs_to_check[@]}"; do
        files_json+=","
        if [[ -d "$dir" ]]; then
            files_json+="\"$dir\":true"
        else
            files_json+="\"$dir\":false"
        fi
    done

    files_json+="}"
    echo "$files_json"
}

# ============================================================================
# Design System Detection
# ============================================================================

# Newline-separated list on stdin -> sorted, de-duplicated JSON array.
lines_to_json() {
    local items
    items=$(grep -v '^$' | sort -u || true)
    if [[ -z "$items" ]]; then
        echo "[]"
    else
        printf '%s\n' "$items" | jq -R . | jq -s .
    fi
}

# Project files up to a depth, skipping dependency and build output.
find_project_files() {
    local depth="$1"
    shift
    find . -maxdepth "$depth" \
        \( -name node_modules -o -name .git -o -name dist -o -name build -o -name .next \
           -o -name Pods -o -name vendor -o -name target \) -prune \
        -o -type f \( "$@" \) -print 2>/dev/null | sed 's|^\./||' | sort || true
}

detect_design_system() {
    local deps="" pkg
    # Dependency names from every package.json near the root (monorepo apps included)
    for pkg in $(find_project_files 3 -name package.json); do
        deps+=$(jq -r '((.dependencies // {}) + (.devDependencies // {}) + (.peerDependencies // {})) | keys[]' "$pkg" 2>/dev/null || true)
        deps+=$'\n'
    done

    has_dep() { grep -qx "$1" <<< "$deps"; }
    has_dep_prefix() { grep -q "^$1" <<< "$deps"; }

    local ui="" css="" fonts="" tokens="" components=""

    # UI framework
    has_dep "react" && ui+=$'react\n'
    has_dep "vue" && ui+=$'vue\n'
    has_dep "svelte" && ui+=$'svelte\n'
    has_dep "@angular/core" && ui+=$'angular\n'
    has_dep "solid-js" && ui+=$'solid\n'
    has_dep "preact" && ui+=$'preact\n'
    has_dep "lit" && ui+=$'lit\n'
    has_dep "next" && ui+=$'nextjs\n'
    has_dep "nuxt" && ui+=$'nuxt\n'
    has_dep "@sveltejs/kit" && ui+=$'sveltekit\n'
    has_dep "astro" && ui+=$'astro\n'
    has_dep "@remix-run/react" && ui+=$'remix\n'
    has_dep "react-native" && ui+=$'react-native\n'
    has_dep "expo" && ui+=$'expo\n'
    has_dep "electron" && ui+=$'electron\n'
    has_dep_prefix "@tauri-apps/" && ui+=$'tauri\n'
    if [[ -f "pubspec.yaml" ]] && grep -q "sdk: flutter" pubspec.yaml 2>/dev/null; then
        ui+=$'flutter\n'
    fi
    local gradle_files
    gradle_files=$(find_project_files 3 -name 'build.gradle' -o -name 'build.gradle.kts')
    if [[ -n "$gradle_files" ]] && grep -q "androidx.compose" $gradle_files 2>/dev/null; then
        ui+=$'jetpack-compose\n'
    fi
    local swift_files
    swift_files=$(find_project_files 4 -name '*.swift')
    if [[ -n "$swift_files" ]] && grep -q "^import SwiftUI" $swift_files 2>/dev/null; then
        ui+=$'swiftui\n'
    fi

    # CSS system
    has_dep "tailwindcss" && css+=$'tailwind\n'
    has_dep "styled-components" && css+=$'styled-components\n'
    has_dep_prefix "@emotion/" && css+=$'emotion\n'
    { has_dep "sass" || has_dep "node-sass"; } && css+=$'sass\n'
    has_dep "less" && css+=$'less\n'
    has_dep_prefix "@vanilla-extract/" && css+=$'vanilla-extract\n'
    has_dep_prefix "@stitches/" && css+=$'stitches\n'
    has_dep "unocss" && css+=$'unocss\n'
    has_dep "@pandacss/dev" && css+=$'panda\n'
    has_dep "bootstrap" && css+=$'bootstrap\n'
    has_dep "bulma" && css+=$'bulma\n'
    [[ -n "$(find_project_files 5 -name '*.module.css' -o -name '*.module.scss')" ]] && css+=$'css-modules\n'

    # Fonts: packaged families, next/font and Google Fonts families, pubspec families, font files
    fonts+=$(grep -E '^@fontsource(-variable)?/' <<< "$deps" | sed -E 's|^@fontsource(-variable)?/||' || true)
    fonts+=$'\n'
    local src_files
    src_files=$(find_project_files 5 -name '*.ts' -o -name '*.tsx' -o -name '*.js' -o -name '*.jsx' \
        -o -name '*.html' -o -name '*.css' -o -name '*.scss' -o -name '*.vue' -o -name '*.svelte' | head -n 500)
    if [[ -n "$src_files" ]]; then
        fonts+=$(grep -hoE "import \{[^}]*\} from ['\"]next/font/google" $src_files 2>/dev/null \
            | sed -E 's/import \{([^}]*)\}.*/\1/' | tr ',' '\n' | sed -E 's/^ +| +$//g; s/ as .*//' || true)
        fonts+=$'\n'
        fonts+=$(grep -hoE "fonts\.googleapis\.com/css2?\?family=[A-Za-z0-9+]+" $src_files 2>/dev/null \
            | sed -E 's/.*family=//; s/\+/ /g' || true)
        fonts+=$'\n'
    fi
    if [[ -f "pubspec.yaml" ]]; then
        fonts+=$(grep -E '^[[:space:]]*- family:' pubspec.yaml 2>/dev/null | sed -E 's/.*family:[[:space:]]*//' || true)
        fonts+=$'\n'
    fi
    fonts+=$(find_project_files 5 -name '*.woff2' -o -name '*.woff' -o -name '*.ttf' -o -name '*.otf' | head -n 20)

    # Token files
    tokens=$(find_project_files 5 -name 'tailwind.config.*' -o -name '*tokens*.json' -o -name '*tokens*.js' \
        -o -name '*tokens*.ts' -o -name 'style-dictionary.config.*' -o -name 'panda.config.*' \
        -o -name 'uno.config.*' -o -name 'theme.ts' -o -name 'theme.js' -o -name 'theme.dart' \
        -o -name '_variables.scss' -o -name 'variables.css' | head -n 20)

    # Component library
    has_dep "@mui/material" && components+=$'mui\n'
    has_dep "@chakra-ui/react" && components+=$'chakra\n'
    has_dep "antd" && components+=$'antd\n'
    has_dep_prefix "@radix-ui/" && components+=$'radix\n'
    if [[ -f "components.json" ]] && grep -q "ui.shadcn.com" components.json 2>/dev/null; then
        components+=$'shadcn\n'
    fi
    has_dep_prefix "@headlessui/" && components+=$'headlessui\n'
    has_dep "@mantine/core" && components+=$'mantine\n'
    has_dep "react-bootstrap" && components+=$'react-bootstrap\n'
    has_dep "vuetify" && components+=$'vuetify\n'
    has_dep "element-plus" && components+=$'element-plus\n'
    has_dep "primevue" && components+=$'primevue\n'
    has_dep "primereact" && components+=$'primereact\n'
    has_dep "daisyui" && components+=$'daisyui\n'
    has_dep "@shopify/polaris" && components+=$'polaris\n'
    has_dep "react-native-paper" && components+=$'react-native-paper\n'

    DESIGN_SYSTEM=$(jq -n \
        --argjson ui "$(lines_to_json <<< "$ui")" \
        --argjson css "$(lines_to_json <<< "$css")" \
        --argjson fonts "$(lines_to_json <<< "$fonts")" \
        --argjson tokens "$(lines_to_json <<< "$tokens")" \
        --argjson components "$(lines_to_json <<< "$components")" \
        '{ui_frameworks: $ui, css_systems: $css, fonts: $fonts, token_files: $tokens, component_libraries: $components}')
}

# ============================================================================
# Main
# ============================================================================

detect_project_type
detect_frameworks
detect_orms
detect_architecture
detect_cicd
detect_design_system
FILES_FOUND=$(detect_files)

# Output JSON
cat <<EOF
{
  "project_type": "$PROJECT_TYPE",
  "package_manager": "$PACKAGE_MANAGER",
  "frameworks": $FRAMEWORKS,
  "orms": $ORMS,
  "architecture": $ARCHITECTURE,
  "ci_cd": $CI_CD,
  "design_system": $DESIGN_SYSTEM,
  "files_found": $FILES_FOUND,
  "scanned_path": "$(pwd)"
}
EOF
