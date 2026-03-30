# Spécifications des Interfaces Utilisateur (UI) - PowerAlert

Ce document décrit en détail les différentes interfaces graphiques (UI) prévues pour l'application PowerAlert afin d'accompagner la phase de design sur Stitch.

## 1. Fenêtre Principale (Tableau de bord / Dashboard)

Il s'agit de la fenêtre unique de l'application (basée sur Astro), qui a pour but d'être minimaliste, moderne et facile à lire. **Cette fenêtre doit avoir une taille fixe (non redimensionnable par l'utilisateur)** étudiée pour contenir exactement tous les éléments de contrôle sans espace vide inutile (par exemple 400x500px, à affiner selon le design).

### Contenu :
*   **En-tête (Header) :**
    *   Titre de l'application ("PowerAlert").
    *   (Optionnel) Un bouton pour minimiser la fenêtre dans la zone de notification (le classique "Réduire").
*   **Indicateur de Batterie (Zone Centrale) :**
    *   **Niveau de charge :** Un grand texte ou une jauge circulaire/horizontale affichant clairement le pourcentage actuel (ex: "85%").
    *   **État de connexion :** Une icône ou un texte (ex: ⚡ "En charge", ou "Sur batterie") indiquant si le chargeur est branché ou non.
*   **Zone de Configuration (Paramètres) :**
    *   **Seuil de charge maximale (Débrancher) :**
        *   Un champ de saisie numérique (input number) ou un curseur (slider) permettant la modification.
        *   Label : "Alerte de charge maximale (%)".
        *   Valeur par défaut : 100.
    *   **Seuil de batterie faible (Brancher) :**
        *   Un champ de saisie numérique ou un curseur.
        *   Label : "Alerte de batterie faible (%)".
        *   Valeur par défaut : 50.
    *   **Autorun (Démarrage système) :**
        *   Un bouton interrupteur (Switch / Toggle).
        *   Label : "Lancer PowerAlert au démarrage de Windows".
*   **Pied de page (Footer) :**
    *   Un texte discret "Paramètres sauvegardés automatiquement" (ou un bouton "Sauvegarder" si la sauvegarde automatique n'est pas voulue).
    *   Un message indiquant que la fermeture de la fenêtre réduit l'application en arrière-plan.

## 2. Menu Contextuel (System Tray / Zone de notification)

L'application tournant en arrière-plan, une icône est présente dans la zone de notification (près de l'horloge Windows). La liste ci-dessous décrit le menu qui s'ouvre lors d'un clic droit sur cette icône.

### Contenu :
*   **Titre Menu :** "PowerAlert - [Niveau actuel de la batterie %]" (Texte grisé, non cliquable).
*   **Séparateur**
*   **Action 1 :** "Ouvrir l'application" (Pour ramener la fenêtre principale au premier plan).
*   **Action 2 :** "Suspendre les alertes" (Optionnel : Permet de désactiver temporairement les notifications sonores/visuelles sans fermer l'application).
*   **Séparateur**
*   **Action 3 :** "Quitter" (Ferme définitivement l'application et arrête la surveillance en arrière-plan).

## 3. Fenêtres d'Alerte Personnalisées (Custom Toasts / Pop-ups)

Il s'agit de petites fenêtres secondaires (conçues en Astro) générées spécifiquement par l'application pour s'afficher lors du franchissement des seuils d'alerte, remplaçant ainsi les notifications basiques natives de l'OS. Elles doivent être harmonieuses avec le design system (Dark mode, glassmorphism, bords arrondis).

### Contenu commun des fenêtres d'alerte :
*   Design rectangulaire compact, s'affichant typiquement en bas à droite de l'écran (hors zone d'interaction principale).
*   Une icône d'état (Batterie pleine ou Batterie vide).
*   Bouton de fermeture discrèt (Croix) ou bouton d'action principal (ex: "J'ai compris").
*   Fermeture manuelle requise une fois l'action effectuée (brancher ou débrancher) ou sur clic.

### A. Alerte de Charge Maximale (Débrancher)
*   **Icône & Thème :** Couleur d'accentuation Bleue (#4da8ff) ou Verte, icône claire de batterie 100%.
*   **Titre :** Batterie pleine !
*   **Message :** "Le niveau a atteint [100]%. Débranchez votre chargeur pour préserver la durée de vie de la batterie."

### B. Alerte de Batterie Faible (Brancher)
*   **Icône & Thème :** Couleur d'accentuation Rouge/Orange (#f56c42), icône d'urgence.
*   **Titre :** Batterie faible !
*   **Message :** "La capacité restante est descendue à [50]%. Branchez votre ordinateur dès que possible."
