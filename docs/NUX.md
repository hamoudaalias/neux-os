Documentation Technique : NEUX OS (v2.0)
Le Système d'Exploitation Augmenté par IA

## Table des Matières

1. [Informations Projet](#informations-projet)
2. [Architecture Hybride](#architecture-hybride)
3. [Noyau Neural Core](#1-architecture-du-noyau--le-neural-core)
4. [LSFS - Système de Fichiers Sémantique](#2-système-de-fichiers--latent-space-file-system-lsfs)
5. [Gestion de la Mémoire](#3-gestion-de-la-mémoire--cognitive-ram)
6. [Liquid UI](#4-interface-utilisateur--liquid-ui)
7. [Sécurité & Chiffrement](#5-sécurité--confidentialité)
8. [Stratégie d'Implémentation](#6-stratégie-dimplémentation-progressive)
9. [Choix Techniques](#7-choix-techniques)
10. [Boot & Hardware](#8-boot--hardware-abstraction)
11. [Stack Réseau](#9-stack-réseau)
12. [Statut d'Implémentation](#10-statut-dimplémentation-v10)
13. [Pivot Stratégique](#11-pivot-stratégique--focus-produit)
14. [LSFS Centric](#12-lsfs-centric-version-produit-réalisable)
15. [Go-to-Market](#14-go-to-market--stratégie-produit)

---

## Informations Projet

| Champ | Valeur |
|-------|--------|
| **Nom** | NEUX OS |
| **Créateur** | Hamouda ALIAS |
| **_langage principal** | Rust |
| **License** | GPLv2 (noyau) + Apache 2.0 (userspace) |
| **Statut** | Prototype / Document |

## Historique des Versions

| Version | Date | Changements majeurs |
|---------|------|-------------------|
| v1.0 | Avril 2026 | Document initial |
| v1.9 | Mai 2026 | Sécurité, stockage, Neural Rendering, corrections |
| v2.0 | Juin 2026 | Publication GitHub, PoC Rust |

> **Version actuelle : v2.0**
> Le projet est sur GitHub : https://github.com/hamoudaalias/neux-os

## GitHub

Le projet est désormais public :

- **Repository** : https://github.com/hamoudaalias/neux-os
- **README** : Version anglaise courte
- **Code** : Structure PoC Rust (CLI index/search)

## Architecture Hybride (Fonctionne avec ce qui est disponible)

NEUX exploite à 100% les ressources disponibles. Il s'adapte automatiquement au hardware détecté.

### Priorité d'Exploitation
1. **GPU** (Graphics Processing Unit) - Primary, CUDA/ROCm/Vulkan
2. **NPU** (Neural Processing Unit) - Bonus si disponible
3. **CPU** (Central Processing Unit) - Fallback robuste
4. **RAM** (Random Access Memory) - Dernier recours

### Features par Configuration

| Hardware | Tensor Ops | Latent Search | Predictive | UI Generation |
|----------|-----------|---------------|------------|---------------|
| NPU + VRAM | 100% (natif) | 100% | 100% | 100% neural |
| GPU + VRAM | 90% (CUDA) | 95% | 90% | 80% neural |
| GPU only | 70% | 80% | 70% | 60% hybrid |
| CPU only | 40% | 50% | 40% | 30% classic |
| RAM only | 10% | 20% | 10% | classic |

### Configuration Requise (Légère & Économie)

#### Configuration Minimale (Mode Neural)
- **GPU** avec **4GB VRAM** minimum
- **CPU** avec support AVX2/NEON
- **8GB RAM** système
- **50GB SSD NVMe**

#### Configuration Minimale (Mode Classic)
- **CPU** avec support SIMD (AVX2, NEON)
- **4GB RAM**
- **20GB SSD**

#### Configuration Optimale
- **GPU** performant (RTX 4070+ ou équivalent)
- **NPU** dédié - optionnel
- **16GB+ VRAM**
- **16GB+ RAM**
- **512GB+ NVMe Gen4**

> **Note** : GPU-first. NPU optionnel. Le système fonctionne sans NPU.

### Architectures Supportées
- ARM Neural (Apple Silicon, Graviton-N)
- x64 avec NPU/GPU discret (Intel AI Boost, AMD XDNA, NVIDIA, AMD)
- RISC-V vectoriel
- NPU dédiés USB-C (eGPU neural)

---

## 1. Architecture du Noyau : Le "Neural Core"

**NEUX est hybride à la base.** Le CPU reste indispensable pour les tâches qui ne peuvent pas être vectorisées.

### Vue d'Ensemble de l'Architecture

```
┌────────────────────────────────────────────────────────────────────────────┐
│                            NEUX OS ARCHITECTURE                             │
├────────────────────────────────────────────────────────────────────────────┤
│                                                                            │
│   ┌──────────────────────────────────────────────────────────────────────┐   │
│   │                    COUCHE UTILISATEUR (USERSPACE)                 │   │
│   │  ┌─────────────┐  ┌─────────────┐  ┌─────────────────────────┐   │   │
│   │  │ Liquid UI  │  │ LSFS API   │  │ Intent Firewall     │   │   │
│   │  │ Generator │  │ (semantic)│  │ (permissions)    │   │   │
│   │  └─────┬─────┘  └─────┬─────┘  └────────┬────────┘   │   │   │
│   │        │              │                   │           │   │   │
│   │  ┌─────┴─────────────────────┴───────────┴────────────┐   │   │
│   │  │              NEURAL DAEMON (Rust)                    │   │   │
│   │  │  - ONNX Runtime    - Tensor Pipeline   - Intent Parser  │   │   │
│   │  └────────────────────────┬──────────────────────────────┘   │   │
│   └──────────────────────────┼──────────────────────────────────────┘   │
│                              │                                               │
│   ┌──────────────────────────┼──────────────────────────────────────┐   │
│   │                    COUCHE MÉTADONNÉES (METADATA LAYER)        │   │
│   │  ┌─────────────────────────────────────────────────────────┐   │   │
│   │  │    LSFS Index (_embeddings + shadow metadata)      │   │   │
│   │  └────────────────────────┬────────────────────────┘   │   │
│   └──────────────────────────┼──────────────────────────────────────┘   │
│                              │                                               │
│   ┌──────────────────────────┼──────────────────────────────────────┐   │
│   │                    COUCHE STOCKAGE (STORAGE LAYER)                │   │
│   │  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐   │   │
│   │  │ VRAM        │  │ SSD NVMe    │  │HDD cold      │   │   │
│   │  │(vecteurs)   │  │(index)       │  │(archive)     │   │   │
│   │  └──────────────┘  └──────────────┘  └──────────────┘   │   │
│   └───────────────────────────────────────────────────────────────┘   │
│                                                                            │
│   ┌────────────────────────────────────────────────────────────────────┐ │
│   │                    COUCHE SYSTÈME (KERNEL)                        │ │
│   │  ┌───────────────────┬───────────────────┬────────────────────┐   │ │
│   │  │   COGNITIVE      ││   CLASSIQUE      ││   ABSTRACTION     │   │ │
│   │  │   (NPU/GPU)     ││    (CPU)        ││   HARDWARE      │   │ │
│   │  │ - Synaptic      ││ - Interruptions ││ - Drivers      │   │ │
│   │  │   Scheduler    ││ - I/O          ││ - Device Tree │   │ │
│   │  │ - Zero-Copy    ││ - Drivers     ││ - Power Mgmt  │   │ │
│   │  │   Path        ││ - Scheduling  ││                │   │ │
│   │  └───────────────────┴───────────────────┴────────────────────┘   │ │
│   │                                                                  │ │
│   └────────────────────────────────────────────────────────────────────┘ │
└────────────────────────────────────────────────────────────────────────────┘
```

### Structure Hybride (Dual-Mode Kernel)

```
┌─────────────────────────────────────────┐
│         Couche Cognitive (NPU/GPU)      │
│  - Synaptic Scheduler (intentions)       │
│  - Liquid UI generation                 │
│  - Semantic search                    │
├─────────────────────────────────────────┤
│         Couche Classique (CPU)           │
│  - Interruptions & I/O                 │
│  - Drivers périphériques              │
│  - Gestion énergie                   │
│  - Scheduling Temps Réel             │
└─────────────────────────────────────────┘
```

### Couche Classique (CPU requis)

Le CPU gère :
- **Interruptions matérielles** (clavier, réseau,'horloge)
- **I/O fichiers binaires** (accès disque exact)
- **Drivers** (périphériques)
- **Gestion alimentation** (veille, réveil)
- **Branching conditionnel** (if/else complexes)

> **Zéro NPU = OK.** La couche cognitive est désactivée, l'OS fonctionne comme un OS classique.

### Couche Cognitive (NPU/GPU optionnel)

**Neural Task Hinting** : Pour éviter la confusion avec un scheduler temps réel, cette couche ne fait pas de scheduling dur. Elle suggère des priorités de tasks aux processus en analysant les patterns d'utilisation. Le scheduler classique (CFS/EEVDF) reste responsable de l'ordonnancement réel.

> **Limitation** : L'inférence réseau neuronal ne peut pas être utilisée pour du scheduling temps réel (latence ~10-100ms). Cette couche est uniquement informative et suggestionnelle.

**Zero-Copy Path** : Les flux capteurs (caméra/micro) → NPU directement pour inférence rapide, sans copies RAM.

**Tensor Orchestration** : Les tenseurs actifs restent en VRAM. L'OS ne monopolise pas la VRAM pour les tâches système.

---

## 2. Système de Fichiers : "Latent Space File System" (LSFS)

**Le LSFS est une couche d'indexation, pas un remplacement du stockage binaire.**

### Architecture à Deux Couches

```
┌────────────────────────────────────────┐
│  Layer 1 : Métadonnées Latentes         │
│  - Embeddings (shadow metadata)          │
│  - Index sémantique                    │
│  - Auto-déduplication (optionnelle)   │
├────────────────────────────────────────┤
│  Layer 2 : Stockage Binaire            │
│  - Fichiers intacts (ext4/ZFS)        │
│  - Exactitude garantie               │
│  - Pas de transformation             │
└────────────────────────────────────────┘
```

### Synchronisation de l'Index (Metadata Shadowing)

Comment garder l'index sémantique à jour quand les fichiers changent ?

#### Méthode 1 : FileWatcher (Temps Réel)

```
┌─────────────────────────────────────────────────────┐
│  FileWatcher (inotify Linux / FSEvents macOS)       │
├─────────────────────────────────────────────────────┤
│  Événement detecté → Queue async → Re-index only   │
│  - CREATE : index nouveau fichier                 │
│  - MODIFY : mise à jour embedding                  │
│  - DELETE : suppression embedding                   │
├─────────────────────────────────────────────────────┤
│  Performance : ~1-5% CPU max, batch processing    │
│  Délai max : 500ms après modification              │
└─────────────────────────────────────────────────────┘
```

#### Méthode 2 : Polling (Fallback)

Si FileWatcher non disponible :
- Scan incrémental toutes les 5 min (configurable)
- Scan full hebdo (index complet)

#### Méthode 3 : User-Triggered

```
# Re-index manuel
neux index --sync ~/Documents
neux index --full  # Scan complet
```

#### Détection de Désynchronisation

| Méthode | Détection | Action |
|---------|-----------|--------|
| Checksum fichier | Comparaison hash | Re-index only si différent |
| Timestamp | mtime modifié | Re-index |
| Manual | Commande explicite | Force re-index |

> **Important** : Les embeddings sont générés à partir du fichier original.
> Si fichier modifié via terminal → FileWatcher détecte → re-index automatique.
> L'index reste toujours un "shadow" (copie métadonnée), jamais le fichier original.

### Règles Critiques

| Type de Fichier | Vectorisable? | Fusion Auto? |
|----------------|--------------|-------------|
| Texte, Images, Audio | ✅Oui | ⚠️Optionnel |
| Binaire exécutable | ❌Non | ❌Jamais |
| Clé de chiffrement | ❌Non | ❌Jamais |
| Config système | ❌Non | ❌Jamais |
| Contrat/juridique | ⚠️Avertissement | ❌Jamais |

> **Danger!** L'auto-déduplication est **désactivée par défaut**. Activable uniquement pour les fichiers personnels (notes, photos).

### Commandes

```bash
# Recherche sémantique (ouvre le fichier exact)
RETRIEVE("réunion budget mars")

# Recherche classique
ls /home/user/docs

# Mode hybridé
ls /home/user/docs --semantic-rank
```

---

## 3. Gestion de la Mémoire : "Cognitive RAM"

**Le CPU reste prioritaire pour les tâches système.**

### Hierarchical Memory (Smart Allocation)

1. **VRAM** → Applications IA utilisateur (priorité haute)
2. **RAM système** → OS, drivers, cache système
3. **SSD** → Vecteurs dormants, swap intelligent
4. **HDD** → Archivecold (indexOnly)

### Règles Mémoire

- **OS jamais enVRAM exclusive** : Maximum 6GB VRAM pour le système
- **Fallback propre** : Si VRAM saturée → vecteurs → SSD (pas de crash)
- **Predit pas, libère** : Le paging prédictif utilise des probabilités légères, pas des modèles lourds

---

## 4. Interface Utilisateur : "Liquid UI"

**Généré dynamiquement, mais avec support apps professionnels.**

### Dual Interface Mode

```
┌─────────────────────────────────────────┐
│         Mode Éphémère (Micro-Apps)       │
│  - Tâches quotidiennes générées        │
│  - UI légère, temporaire              │
│  - Créée à la volée par le NPU         │
├─────────────────────────────────────────┤
│         Mode Canvas (Apps Pro)         │
│  - Logiciels pré-compilés              │
│  - Interface complète stable          │
│  - Rendu classique (Qt/WebGPU)         │
└─────────────────────────────────────────┘
```

### Pourquoi les deux modes?

| Tâche | Mode Éphémère | Mode Canvas |
|-------|---------------|-------------|
| Lire un mail | ✅Généré | ❌Inutile |
| Chercher fichier | ✅Généré | ❌Inutile |
| Montage vidéo | ❌Impossible | ✅Prévu |
| Développement code | ❌ Trop complexe | ✅IDE complet |
| Gaming 3D | ❌ | ✅Native |

> **L'IA génère des Micro-Apps pour les tâches simples.** Les apps professionnels tournent en mode Canvas classique.

### Mécanisme de Génération (Détails Techniques)

L'intent utilisateur → modèle multimodal → composant généré.

```
┌─────────────────────────────────────────────────────────┐
│  Flux de génération Liquid UI                             │
│  ┌─────────────┐    ┌─────────────┐    ┌────────────┐  │
│  │ Intent      │───▶│ Phi-3-Vision│───▶│ Prompt    │  │
│  │ utilisateur│    │ (7B quant)  │    │ structuré │  │
│  └─────────────┘    └─────────────┘    └────────────┘  │
│         │                                      │         │
│         ▼                                      ▼         │
│  ┌─────────────┐                      ┌────────────┐  │
│  │ Fallback    │◀─────────────────────│ Tauri/    │  │
│  │ Canvas     │                      │ React     │  │
│  │ forcé     │                      │ + WebGPU   │  │
│  └─────────────┘                      └────────────┘  │
└─────────────────────────────────────────────────────────┘
```

**Spécifications** :
- **Modèle** : Phi-3-Vision ou équivalent 7B quantizé (~2-5s)
- **Templates précachés** : Pour les 10 cas usage courante (UI classique ~instantanée)
- **Génération IA** : Uniquement pour cas nouveaux/complexes
- **Mode** : Génération en tâche de fond, pas en hot-path
- **Fallback** : Canvas disponible immédiatement

### Neural Rendering (v2.0+)

Comment l'OS décide visuellement de présenter un résultat ?

#### Architecture de Rendu

```
┌─────────────────────────────────────────────────────┐
│  Neural Rendering Pipeline                            │
├─────────────────────────────────────────────────────┤
│  Intent détectée                                  │
│       ↓                                          │
│  [Template Match] ──→ UI Templates précachés      │
│       ↓ (pas de match)                           │
│  [Phi-3-Vision] ──→ Génération composant         │
│       ↓                                         │
│  [Tauri/WebGPU] ──→ Rendu pixel                 │
└─────────────────────────────────────────────────────┘
```

#### Templates Précachés (v1.0)

Pour les 10 cas d'usage courants :

| Cas | Template | Délai |
|-----|----------|-------|
| Recherche fichier | Liste compacte | < 1ms |
| Preview image | Grid + thumb | < 1ms |
| Preview PDF | Miniature | < 5ms |
| Lecture mail | Carte email | < 1ms |
| Code file | Syntax highlight | < 1ms |
| Command run | Terminal output | < 1ms |
| Settings | Formulaire | < 1ms |
| Search | Résultats liste | < 1ms |
| File info | Carte détails | < 1ms |
| Help | FAQ accordion | < 1ms |

#### Génération IA (v2.0+)

Pour les cas non-couverts par templates :

1. ** Analyse de l'intent** : Petit modèle (100M params) pour classifier le besoin
2. ** Extraction des entités** : Fichiers, dates, personnes impliqués
3. ** Choix du layout** : Liste, grid, forme carte, timeline
4. ** Génération composants** : Via Phi-3-Vision (2-5s)
5. ** Cache du resultat** :Pour réutilisation future

> **Note** : v1.0 utilise uniquement les templates. Génération IA en v2.0+.

**Accessibilité (a11y)** :
- Focus clavier géré par le renderer
- Raccourcis standard préservés
- Sortie ARIA pour lecteurs d'écran
- Mode "Canvas forcé" disponible manuellement

---

## 5. Sécurité & Confidentialité

### Principes

- **L'OS ne "comprend" que les métadonnées**, pas le contenu chiffré
- **Local-first** : Tout reste sur la machine utilisateur par défaut
- **Shadow Semantics** : Les embeddings sont stockés séparément du contenu

### Chiffrement au Repos (Encrypted-at-Rest)

Le chiffrement homomorphe complet (FHE) est encore trop lent pour un OS temps réel en 2026. Voici l'approche pragmatique :

```
┌─────────────────────────────────────────────────────────┐
│  Encrypted-at-Rest v1                               │
│  - Chiffrement AES-256-GCM des embeddings              │
│  - Index chiffré stocké sur SSD                     │
│  - Décryptage en VRAM session-active uniquement    │
├─────────────────────────────────────────────────────────┤
│  FHE Complet v2 (2028+)                           │
│  - Chiffrement homomorphe sur embeddings            │
│  - Recherche sans jamais déchiffrer               │
└─────────────────────────────────────────────────────────┘
```

> **Note** : Ce n'est PAS du searchable encryption. C'est du chiffrement au repos standard.
> Les embeddings sont chiffrés sur disque (AES-256-GCM) mais doivent être déchiffrés en VRAM pour calculer la similarité cosinus. Le vrai FHE sera évalué pour v2.

**Session Sécurisée** :
- Embeddings encryptés avec AES-256-GCM
- Clé stockée dans TPM ou Secure Enclave
- Décryptage uniquement en VRAM pendant session active
- Effacement VRAM sur lock/suspend
- Pas de swap des embeddings déchiffrés

### Intent Firewall (Permissions granulaires)

| Permission | Description | Risque | Niveau |
|------------|-------------|-------|--------|
| `RETRIEVE_OWN` | Rechercher ses propres fichiers | Faible | Default |
| `RETRIEVE_USER` | Rechercher tous fichiers utilisateur | Moyen | Approbation |
| `RETRIEVE_SYSTEM` | Accéder métadonnées système | Élevé | Admin only |
| `INDEX` | Créer embeddings | Faible | Default |

**Implémentation** :
- Auth via UID/GID système Linux classique
- Rate limiting : 10 req/s par UID
- Audit log toutes les requêtes
- Blocage automatique mots-clés ("password", "bank", "secret")

---

## 6. Stratégie d'Implémentation Progressive

NEUX ne peut pas être construit en une fois. Voici la roadmap réaliste :

### Phase 1 : Prototype (MVP)
**Objectif** : Valider le flux "intent → micro-app"
**Délai estimé** : 3-6 mois
**Livrable** : LSFS CLI fonctionnel + indexeur sémantique

```
Stack technique :
- Base : Linux kernel (fork ou container)
- LSFS : FUSE userspace
- UI : Tauri + WebGPU neural renderer
- NPU : Python/TensorFlow via RPC (pas driver natif)

Fonctionne sur :
- Tout PC avec 8GB RAM + GPU (CUDA/ROCm)
```

### Phase 2 : LSFS Production
**Objectif** : Système de fichiers sémantique opérationnel
**Délai estimé** : 6-12 mois

```
- Module noyau Linux (ext4 + index.vectoriel)
- Embeddings.storage sur SSD NVMe
- API REST pour intents
- Rate limiting kernel-space
```

### Phase 3 : Neural Kernel Module
**Objectif** : Couche cognitive en userspace stable
**Délai estimé** : 12-18 mois

```
- Daemon neural (Rust)
- Tensor pipeline (ONNX Runtime)
- Liquid UI generator
- Intent Firewall en Go/Rust
```

### Phase 4 : Noyau Hybride (V1.0)
**Objectif** : Fork Linux avec module NeuralCore
**Délai estimé** : 5-7 ans (après validation product/market fit des phases 1-3)

> **Réaliste** : Comme Fuchsia OS (Google) a pris 6+ ans avec des centaines d'ingénieurs, 5-7 ans avec une petite équipe est plus crédible.

```
Option recommandée : Fork Linux + module NeuralCore
- Maintien des drivers existants
- Intégration progressive du scheduler neural

Cible hardware :
- Priorité 1: x64 + NVIDIA/AMD (CUDA/ROCm)
- Priorité 2: Apple Silicon (ANE via Metal)
- Priorité 3: x64 + NPU Intel/AMD (OpenVINO)
- Priorité 4: RISC-V + NPU (phase v2)
```

---

## 7. Choix Techniques

### Langage de Développement

| Composant | Langage | Justification |
|-----------|--------|---------------|
| Noyau kernel | Rust | Mémoire sécurisée, performance |
| Neural daemon | Rust/Python | ONNX bindings, rapid prototyping |
| LSFS userspace | Rust | Performance I/O |
| Liquid UI | TypeScript/WebGPU | Rendering flexible |
| Drivers | C/Rust | Hardware abstraction |

### Base Noyau

**Décision** : Fork Linux + module NeuralCore en userspace (phases 1-3)

```
┌─────────────────────────────────────────────────────────┐
│  Pourquoi Fork Linux ?                               │
│  - Drivers existants (wifi, gpu, usb...)           │
│  - Compatibilité applicative                         │
│  - Équipe peut se concentrer sur NeuralCore       │
├─────────────────────────────────────────────────────────┤
│  Perspectives v2 (Non inclus en v1.0)            │
│  - Micro-noyau Zircon                              │
│  - From scratch Rust                               │
│  - Seulement après v1.0 stable                  │
└─────────────────────────────────────────────────────────┘
```

### Modèle de Licensing

```
NEUX OS - License Mixte

┌────────────────────────────────────────────┐
│  Composants Open Source                     │
│  - Noyau Linux fork : GPLv2 (obligation)  │
│  - LSFS userspace : Apache 2.0            │
│  - Intent Firewall : Apache 2.0             │
├────────────────────────────────────────────┤
│  Composants Propriétaires (Commerciale)      │
│  - Neural Renderer (algorithmes)           │
│  - Modèles IA pré-entraînés               │
│  - Support entreprise                    │
└────────────────────────────────────────────┘

> **Note** : Un fork Linux doit rester sous GPLv2 (sauf re-license explicite).
> Les composants userspace peuvent utiliser Apache 2.0 ou MIT.
```

## 8. Boot & Hardware Abstraction

### Boot Sequence ( phases 1-3 : Linux-based)

```
UEFI/BIOS → GRUB/Linux → systemd → NEUX daemon (userspace)
                              ↓
                    - Détection GPU/NPU
                    - Chargement index LSFS
                    - Démarrage Liquid UI (optionnel)
```

**Bootloader** : GRUB standard (phase 1-3)
**Init system** : systemd (compatibilité)
**Pas de custom bootloader** : Until phase 4

### Hardware Detection

| Composant | Détection | Driver |
|----------|---------|--------|
| GPU | nvme/pci | NVIDIA driver / AMDGPU |
| NPU | pci / usb | openvino /-metal /ANE |
| Stockage | kernel | ext4 / btrfs / zfs |

---

## 9. Stack Réseau

> **Local-first** : NEUX est conçu pour fonctionner sans réseau.

### Option A : NetworkManager (phase 1-3)
- Utilisation de NetworkManager ou systemd-networkd
- Pas de stack réseau custom

### Option B : Custom (phase 4)
- only après validation product/market fit

> **Note** : Les intents NEUX sont locaux par défaut. sync cloud optionnel via API tierce.

---

## 10. Statut d'Implémentation (v1.0)

| Composant | Statut | Note |
|-----------|-------|------|
| LSFS concept | ✅ Validated | Théorie incontournée |
| LSFS PoC | 🔄 In Progress | Prototype FUSE |
| Intent API | 🔄 In Progress | gRPC definition |
| Liquid UI | 🟡 Planned | WebGPU tests |
| Intent Firewall | 🟡 Planned | Rate limiting |
| Neural Core | 🟡 Planned | Module design |
| Noyau hybride | 🟡 Planned | Phase 4 |

---

## 11. Pivot Stratégique : Focus Produit

Suite aux critiques, le projet nécessite un recentrage. Trois directions possibles :

### Option A : Research (Vision long terme - AGI OS)
```
Objectif : Système complet neuronal
Risque : 5-10 ans, peut-être jamais terminé
Équipe needed : 10+ personnes
```

### Option B : Produit utilisable rapidement (Recommandé)
```
Objectif : LSFS en userspace, livrable en 6-12 mois
- Index sémantique local (Linux/macOS)
- CLI simple + plugin Finder/Explorer
- API REST pour intents
Priorité : Productivité immédiate
```

### Option C : Proof technique (Fundraising)
```
Objectif : Démontrer la viabilité technique
- Prototype LSFS fonctionnel
- Démo Liquid UI
- Viser seed round ou recrutement
```

> **Décision recommandée** : Option B (LSFS produit)
> - Risque faible
> - Valeur immédiate prouvable
> - Stack technique maîtrisable

---

## 12. LSFS Centric (Version Produit Réalisable)

Cette version se concentre uniquement sur le système de fichiers sémantique.

### Stack Technique (LSFS v1)

```
Couche 1 : Indexeur sémantique (Rust + ONNX)
- Embeddings via sentence-transformers (all-MiniLM-L6-v2)
- Stockage index sur SSD (FAISS ou LanceDB)

Couche 2 : CLI + API
- Commandes : retrieve, index, search
- API gRPC pour intégration tierce
```

### Format de Stockage des Embeddings

Pour une recherche rapide (< 10ms), le format de stockage est critique.

#### Options de Stockage

| Solution | Latence | Utilisation Mémoire | Cross-Platform | Recommandation |
|----------|---------|---------------------|---------------|----------------|
| FAISS (Flat) | ~5-10ms | Élevée | ✅ | v1.0 (simple) |
| FAISS (IVF) | ~1-5ms | Moyenne | ✅ | v1.1 (production) |
| LanceDB | ~1-3ms | Faible | ✅ | v1.2 (final) |
| Milvus | ~1-5ms | Faible | ❌ (Docker) | ❌ |

> **Choix v1.0** : FAISS Flat (simple, robuste, pas de dépendance Docker)

#### Optimisations de Recherche

```
┌─────────────────────────────────────────────────────┐
│  Pipeline de Recherche Optimisé                    │
├─────────────────────────────────────────────────────┤
│  1. Embedding query (ONNX, ~1ms)               │
│  2. Top-K approximate (FAISS, ~5ms)             │
│  3. Reranking (cross-encoder, ~2ms)             │
│  4. Retour résultats triés                    │
├─────────────────────────────────────────────────────┤
│  Total : ~8-10ms pour 100K fichiers            │
└─────────────────────────────────────────────────────┘
```

#### Compression des Embeddings

Pour réduire la taille sur disque :

- **Quantization int8** : ~50% réduction taille, perte précision < 2%
- **PCA** : Réduction dimensions si nécessaire
- **Lazy loading** : Chargement uniquement des vecteurs nécessaires

#### Métadonnées Index

Chaque entrée stockée :

```
Entry {
  file_path: String,
  embedding: Vec<float32>,  // 384 dimensions (all-MiniLM-L6-v2)
  file_hash: String,        // SHA-256 pour détection changement
  last_modified: Timestamp,
  file_type: String,
  size_bytes: u64
}
```

Taille par entry : ~1.6KB (384 × 4 bytes + métadonnées)

### Commandes LSFS v1

```bash
# Indexer un dossier
neux index ~/Documents

# Recherche sémantique
neux search "réunion budget"

# Mode strict (résultats déterministes)
neux search "réunion budget" --strict

# Versionnage embeddings
neux index --update
```

### Limitations Connues

- Pas de génération UI
- Pas de scheduler neural
- NPU optionnel (GPU/CPU fallback)
- Recherche sémantique sans chiffrement v1

### Différenciation vs Concurrence

| Outil | CE que fait LSFS | Différence clé |
|-------|------------------|---------------|
| Microsoft Recall | Index Windows local | LSFS : open source, CLI, Linux/macOS |
| Rewind.ai | Recording + recherche | LSFS : index offline, pas de recording |
| Perplexica local | Recherche web sémantique | LSFS : fichiers locaux uniquement (privacy) |
| Everything (Voidtools) | Recherche filename | LSFS : recherche par contenu sémantique |

**CE que LSFS fait différemment** :
- **Open source** (pas de cloud, pas de subscription)
- **Cross-platform** : Linux + macOS + Windows
- **Index sémantique** : pas juste le nom de fichier, mais le contenu
- **CLI-first** : intégration devs, pas d'UI mandatory
- **Rust** : performance, mémoire sécurisée

### Prochaines Étapes (Après v1)

| Version | Feature |
|---------|---------|
| v1.1 | Chiffrement AES-256 |
| v2.0 | Intent API |
| v2.1 | Liquid UI limité |
| v3.0 | Kernel module |

| Risque | Impact | Mitigation |
|-------|--------|------------|
| FHE trop lent | Performance | Chiffrement au repos v1, FHE v2 |
| NPU drivers | Portabilité | Backend CUDA/ROCm/OpenVINO |
| Complexité noyau | Maintenance | Architecture modulaire |
| Intent errors | UX | Fallback Canvas forcé |
| Portabilité | Temps | Priorité Linux puis ARM |

> **Ce projet est ambitieux. Il nécessite une équipe de 5-10 personnes sur 3-5 ans pour atteindre la v1.0.**

---

## 14. Go-to-Market & Stratégie Produit

### Cibles Utilisateurs (Priorisées)

| Cible | Potentiel | Difficulté | Recommandation |
|------|----------|------------|----------------|
| Devs / Power Users | Fort | Faible | ✅ Priorité v1 |
| Entreprise | Fort | Moyenne | v2 |
| Grand Public | Faible | Haute | ❌ |

> **Positionnement v1** : "grep + AI" pour développeurs
> CLI-first, local-only, privacy-first

### Améliorations Produit (Critical)

Pour éviter le piège "me too", ces features sont Obligatoires :

#### 1. Ranking Amélioré

- **Cross-encoder reranking** : résultats rerankés par pertinence
- **Feedback implicite** : les fichiers cliqués remontent dans le ranking
- **Historique** : les recherches récentes prioritaires

#### 2. Explainability

Chaque résultat affiche pourquoi :
```
$ neux search "budget réunion"

/home/user/docs/budget_q1.pdf
  Score: 0.92
  Reasons: "budget" ×3, "réunion" ×2, contexte proche
  Modified: 2 jours
```

#### 3. Indexation Intelligente (Lazy & Progressive)

```
┌─────────────────────────────────────────────────────┐
│  Indexation Strategy v1                              │
├─────────────────────────────────────────────────────┤
│  1. Fichiers récents (7 derniers jours) - PRIO  │
│  2. Fichiers fréquemment ouverts              │
│  3. Autres fichiers (lazy, background)       │
├─────────────────────────────────────────────────────┤
│  Performance : 80% des résultats en 20% temps │
└─────────────────────────────────────────────────────┘
```

### MVP Steps (6-8 semaines)

```
Step 1 : CLI Minimal (2 semaines)
- neux index <dossier>
- neux search <query>
- Sortie textuelle simple

Step 2 : Ranking + Vitesse (1 semaine)
- Cross-encoder lightweight
- Cache des embeddings communs

Step 3 : "Wow Effect" (1 semaine)
- neux search "le pdf où je parle de X avec Y"
- Highlight des passages

Step 4 : Distribution (1 semaine)
- Publication GitHub
- Préparation HN/Reddit
```

### Distribution

| Canal | Priorité | Timing |
|------|---------|--------|
| GitHub + README | Haute | Day 1 |
| Hacker News | Haute | Week 2 |
| Reddit r/rust, r/programming | Moyenne | Week 2 |
| Dev.to | Moyenne | Week 3 |

> **Objectif v1** : 1000 utilisateurs en 3 mois
> **Métrique** : Stars GitHub + usage CLI

### Principes

- **L'OS ne "comprend" que les métadonnées**, pas le contenu chiffré
- **Local-first** : Tout reste sur la machine utilisateur par défaut
- **Shadow Semantics** : Les embeddings sont stockés séparément du contenu

### Protection

| Donnée | Accès Sémantique? | Chiffrement? |
|--------|-------------------|--------------|
| Fichiers locaux | ✅Optionnel | ✅AES-256 |
| Cloud sync | ⚠️Embeddings only | ✅E2EE |
| Métadonnées-système | ❌Non | ❌Non |

- Les embeddings de recherche sont **séparés** du contenu original
- Recherche sémantique possible **sans déchiffrer** le contenu
- Option "Cloaked Mode" : Désactive temporairement le LSFS sémantique

### Chiffrement au Repos (Encrypted-at-Rest)

Le chiffrement homomorphe complet (FHE) est encore trop lent pour un OS temps réel en 2026. Voici l'approche pragmatique :

```
┌─────────────────────────────────────────────────────────┐
│  Encrypted-at-Rest v1                               │
│  - Chiffrement AES-256-GCM des embeddings              │
│  - Index chiffré stocké sur SSD                     │
│  - Décryptage en VRAM session-active uniquement    │
├─────────────────────────────────────────────────────────┤
│  FHE Complet v2 (2028+)                           │
│  - Chiffrement homomorphe sur embeddings            │
│  - Recherche sans jamais déchiffrer               │
└─────────────────────────────────────────────────────────┘
```

> **Note** : Ce n'est PAS du searchable encryption. C'est du chiffrement au repos standard.
> Les embeddings sont chiffrés sur disque (AES-256-GCM) mais doivent être déchiffrés en VRAM pour calculer la similarité cosinus. Le vrai FHE sera évalué pour v2.

```
┌─────────────────────────────────────────────────────────┐
│  Session Déverrouillée (clé en RAM)                      │
│  ┌────────────────────────────────────────────────────┐ │
│  │  Embeddings chiffrés (AES-256-GCM)              │ │
│  │  ↓ déchiffrés en VRAM uniquement session-active      │ │
│  └────────────────────────────────────────────────────┘ │
│  ↓ Recherche possible                                   │
│  Résultats retournés                                     │
├─────────────────────────────────────────────────────────┤
│  Session Verrouillée / Éteint                           │
│  - Base vectorielle chiffrée (AES-256-GCM)              │
│  - Clé uniquement en secure enclave ou TPM             │
│  - Zéro accès en RAM hors session            │
└─────────────────────────────────────────────────────────┘
```

**Règles FHE-Light** :
- Embeddings stockés **toujours chiffrés** sur disque
- Déchiffrement **uniquement** en VRAM pendant session active
- Effacement VRAM session-lock/suspend
- Pas de swap des embeddings déchiffrés

### Resource Contention (NPU/GPU)

**Préemption des Tenseurs** :

| Mode | OS NPU Usage | App NPU Usage | Réservation |
|------|-------------|---------------|-------------|
| Idle | 100% | 0% | OS seul |
| App IA légère | 50% | 50% | Partagé |
| App IA lourde | 0% | 100% | OS dégradé CPU |
| Gaming/3D | 0% | 100% | OS dégradé CPU |

**Règles** :
- L'OS **cède automatiquement** le NPU aux apps demandeuses
- **Seuil configurable** : utilisateur définit minimum neural UI
- **Fallback CPU** : si app prend NPU, OS retourne en mode classic
- **Cœurs réservés** : sur NPU multi-cœurs, 1 cœur réservé pour OS

### Intent Firewall (Permissions LSFS)

**Modèle de permissions granulaire** :

```
┌─────────────────────────────────────────────────────────┐
│                    INTENT FIREWALL                       │
├─────────────────────────────────────────────────────────┤
│  App Mode Canvas → Accès API LSFS via permission       │
│  - PERM_RETRIEVE_OWN :only propres fichiers              │
│  - PERM_RETRIEVE_ALL :admin only                        │
│  - PERM_INDEX :ajouter nouveaux embeddings           │
│  - PERM_DELETE :supprimer embeddings                   │
├─────────────────────────────────────────────────────────┤
│  Rate Limiting                                        │
│  - Max 10 requêtes semantic/second per app           │
│  - Audit log toutes les requêtes                      │
├─────────────────────────────────────────────────────────┤
│  Sandboxing                                            │
│  - Chaque app à son propre index vectoriel            │
│  - Pas d'accès cross-app sans permission explicite     │
└─────────────────────────────────────────────────────────┘
```

**Catégories de Permissions**

| Permission | Description | Risque | Niveau |
|------------|-------------|-------|--------|
| `RETRIEVE_OWN` | Rechercher ses propres fichiers | Faible | Default |
| `RETRIEVE_USER` | Rechercher tous fichiers utilisateur | Moyen | Approbation |
| `RETRIEVE_SYSTEM` | Accéder métadonnées système | Élevé | Admin only |
| `INDEX` | Créer embeddings | Faible | Default |
| `INDEX_FORCE` | Vectoriser fichiers non-autorisés | Élevé | Admin only |
| `LISTEN_AUDIO` | Accéder micro pour intents | Élevé | Opt-in |
| `LISTEN_CAMERA` | Accéder caméra pour intents | Critique | Opt-in |

**Implémentation Permissions**

- Auth via **UID/GID** système Linux classique
- Fichiers "propres" = fichiers appartenants à l'UID du processus
- Tagging sémantique par métadonnées (pas par contenu)
- Rate limiting : 10 req/s par UID (configurable)

---

> **Version actuelle : v1.9**
> Le projet est produit-prêt. Prochaine étape : PoC LSFS.