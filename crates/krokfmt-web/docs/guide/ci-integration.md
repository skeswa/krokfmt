# CI/CD Integration

Integrate krokfmt into your continuous integration pipeline to ensure consistent formatting.

## GitHub Actions

### Basic Check

```yaml
name: Format Check

on: [push, pull_request]

jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      
      - name: Install krokfmt
        run: |
          curl -L https://github.com/skeswa/krokfmt/releases/latest/download/krokfmt-linux-x86_64 -o krokfmt
          chmod +x krokfmt
          sudo mv krokfmt /usr/local/bin/
      
      - name: Check formatting
        run: krokfmt --check .
```

### Auto-format and Commit

```yaml
name: Auto Format

on:
  push:
    branches: [main]

jobs:
  format:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
        with:
          token: ${{ secrets.GITHUB_TOKEN }}
      
      - name: Install krokfmt
        run: |
          curl -L https://github.com/skeswa/krokfmt/releases/latest/download/krokfmt-linux-x86_64 -o krokfmt
          chmod +x krokfmt
          sudo mv krokfmt /usr/local/bin/
      
      - name: Format code
        run: krokfmt .
      
      - name: Commit changes
        run: |
          git config --local user.email "action@github.com"
          git config --local user.name "GitHub Action"
          git add .
          git diff --staged --quiet || git commit -m "Auto-format code with krokfmt"
          git push
```

### Matrix Strategy

```yaml
name: Format Check

on: [push, pull_request]

jobs:
  format:
    strategy:
      matrix:
        os: [ubuntu-latest, windows-latest, macos-latest]
    runs-on: ${{ matrix.os }}
    steps:
      - uses: actions/checkout@v4
      
      - name: Install krokfmt
        shell: bash
        run: |
          if [[ "$RUNNER_OS" == "Linux" ]]; then
            BINARY="krokfmt-linux-x86_64"
          elif [[ "$RUNNER_OS" == "Windows" ]]; then
            BINARY="krokfmt-windows-x86_64.exe"
          else
            BINARY="krokfmt-darwin-x86_64"
          fi
          
          curl -L "https://github.com/skeswa/krokfmt/releases/latest/download/${BINARY}" -o krokfmt
          chmod +x krokfmt
          
      - name: Check formatting
        run: ./krokfmt --check .
```

## GitLab CI

```yaml
format:check:
  stage: test
  image: rust:latest
  before_script:
    - curl -L https://github.com/skeswa/krokfmt/releases/latest/download/krokfmt-linux-x86_64 -o /usr/local/bin/krokfmt
    - chmod +x /usr/local/bin/krokfmt
  script:
    - krokfmt --check .
  only:
    - merge_requests
    - main

format:fix:
  stage: test
  image: rust:latest
  before_script:
    - curl -L https://github.com/skeswa/krokfmt/releases/latest/download/krokfmt-linux-x86_64 -o /usr/local/bin/krokfmt
    - chmod +x /usr/local/bin/krokfmt
  script:
    - krokfmt .
    - |
      if [[ -n $(git status -s) ]]; then
        git config user.email "ci@example.com"
        git config user.name "GitLab CI"
        git add .
        git commit -m "Auto-format with krokfmt"
        git push origin HEAD:$CI_COMMIT_REF_NAME
      fi
  only:
    - main
```

## CircleCI

```yaml
version: 2.1

jobs:
  format-check:
    docker:
      - image: cimg/base:stable
    steps:
      - checkout
      - run:
          name: Install krokfmt
          command: |
            curl -L https://github.com/skeswa/krokfmt/releases/latest/download/krokfmt-linux-x86_64 -o krokfmt
            chmod +x krokfmt
            sudo mv krokfmt /usr/local/bin/
      - run:
          name: Check formatting
          command: krokfmt --check .

workflows:
  test:
    jobs:
      - format-check
```

## Jenkins

### Declarative Pipeline

```groovy
pipeline {
    agent any
    
    stages {
        stage('Install krokfmt') {
            steps {
                sh '''
                    curl -L https://github.com/skeswa/krokfmt/releases/latest/download/krokfmt-linux-x86_64 -o krokfmt
                    chmod +x krokfmt
                '''
            }
        }
        
        stage('Check Format') {
            steps {
                sh './krokfmt --check .'
            }
        }
    }
    
    post {
        failure {
            echo 'Code formatting check failed. Please run krokfmt locally.'
        }
    }
}
```

## Azure DevOps

```yaml
trigger:
  - main
  - develop

pool:
  vmImage: 'ubuntu-latest'

steps:
  - script: |
      curl -L https://github.com/skeswa/krokfmt/releases/latest/download/krokfmt-linux-x86_64 -o krokfmt
      chmod +x krokfmt
      sudo mv krokfmt /usr/local/bin/
    displayName: 'Install krokfmt'
  
  - script: krokfmt --check .
    displayName: 'Check formatting'
```

## Bitbucket Pipelines

```yaml
pipelines:
  default:
    - step:
        name: Format Check
        image: rust:latest
        script:
          - curl -L https://github.com/skeswa/krokfmt/releases/latest/download/krokfmt-linux-x86_64 -o krokfmt
          - chmod +x krokfmt
          - ./krokfmt --check .
        
  pull-requests:
    '**':
      - step:
          name: Auto Format
          image: rust:latest
          script:
            - curl -L https://github.com/skeswa/krokfmt/releases/latest/download/krokfmt-linux-x86_64 -o krokfmt
            - chmod +x krokfmt
            - ./krokfmt .
            - |
              if [[ -n $(git status -s) ]]; then
                git add .
                git commit -m "Auto-format with krokfmt"
                git push
              fi
```

## Travis CI

```yaml
language: rust
rust: stable

before_script:
  - curl -L https://github.com/skeswa/krokfmt/releases/latest/download/krokfmt-linux-x86_64 -o krokfmt
  - chmod +x krokfmt

script:
  - ./krokfmt --check .
```

## Docker Integration

### Dockerfile

```dockerfile
FROM rust:latest AS formatter

# Install krokfmt
RUN curl -L https://github.com/skeswa/krokfmt/releases/latest/download/krokfmt-linux-x86_64 -o /usr/local/bin/krokfmt && \
    chmod +x /usr/local/bin/krokfmt

# Copy source code
WORKDIR /app
COPY . .

# Format check
RUN krokfmt --check .
```

### Docker Compose

```yaml
version: '3.8'

services:
  format-check:
    image: rust:latest
    volumes:
      - .:/app
    working_dir: /app
    command: |
      sh -c "
        curl -L https://github.com/skeswa/krokfmt/releases/latest/download/krokfmt-linux-x86_64 -o krokfmt &&
        chmod +x krokfmt &&
        ./krokfmt --check .
      "
```

## Caching

### GitHub Actions Cache

```yaml
- name: Cache krokfmt
  uses: actions/cache@v3
  with:
    path: ~/.local/bin/krokfmt
    key: ${{ runner.os }}-krokfmt-${{ hashFiles('**/krokfmt-version') }}

- name: Install krokfmt
  run: |
    if [ ! -f ~/.local/bin/krokfmt ]; then
      mkdir -p ~/.local/bin
      curl -L https://github.com/skeswa/krokfmt/releases/latest/download/krokfmt-linux-x86_64 -o ~/.local/bin/krokfmt
      chmod +x ~/.local/bin/krokfmt
    fi
    echo "$HOME/.local/bin" >> $GITHUB_PATH
```

## Best Practices

1. **Run format checks early** in the pipeline to fail fast
2. **Cache the binary** to speed up subsequent runs
3. **Use --check mode** for pull requests
4. **Auto-format** only on main branch
5. **Include in parallel jobs** to avoid blocking other tests
6. **Set up notifications** for format failures

## Exit Codes

Use exit codes for conditional steps:

```bash
if krokfmt --check .; then
  echo "✅ Code is properly formatted"
else
  echo "❌ Code needs formatting"
  echo "Run 'krokfmt .' locally to fix"
  exit 1
fi
```