# 🎹 Piano Virtuel en Rust

Une application de piano interactive en ligne de commande créée avec Rust !

## 🚀 Installation et Lancement

### Prérequis
- Rust installé (https://rustup.rs/)

### Lancer l'application

```bash
cd piano
cargo run
```

## 🎵 Comment jouer

### Octave 1 (Grave)
- **a** = Do (C4)
- **z** = Do# (C#4)
- **e** = Ré (D4)
- **r** = Ré# (D#4)
- **t** = Mi (E4)
- **y** = Fa (F4)
- **u** = Fa# (F#4)
- **i** = Sol (G4)
- **o** = Sol# (G#4)
- **p** = La (A4)
- **q** = La# (A#4)
- **s** = Si (B4)

### Octave 2 (Aigu)
- **d** = Do (C5)
- **f** = Do# (C#5)
- **g** = Ré (D5)
- **h** = Ré# (D#5)
- **j** = Mi (E5)
- **k** = Fa (F5)
- **l** = Fa# (F#5)
- **m** = Sol (G5)

### Contrôles
- **Échap** : Quitter l'application
- **Ctrl+C** : Quitter l'application

## 🎼 Fonctionnalités

✨ Génération de sons en temps réel avec ondes sinusoïdales
✨ Enveloppe ADSR pour des sons plus naturels
✨ 2 octaves complètes (20 notes)
✨ Interface en ligne de commande colorée
✨ Affichage des notes jouées en temps réel

## 🛠️ Technologies utilisées

- **rodio** : Bibliothèque audio pour Rust
- **crossterm** : Gestion des entrées clavier en temps réel
- Génération de sons par synthèse sinusoïdale

## 💡 Améliorations possibles

- Ajouter plus d'octaves
- Implémenter différents instruments (piano, guitare, flûte...)
- Enregistrement et lecture de mélodies
- Support MIDI
- Interface graphique (avec une bibliothèque comme `iced` ou `egui`)

## 📚 Apprentissage Rust

Ce projet est parfait pour apprendre :
- La gestion audio en Rust
- Les événements et l'interaction utilisateur
- Le traitement du signal (génération d'ondes)
- La concurrence (sons joués simultanément)

Amuse-toi bien ! 🎶
