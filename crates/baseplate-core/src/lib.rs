//! Place diffs and turn records.
//!
//! Every agent turn is planned as a [`PlaceDiff`] and shown in full before
//! anything touches Studio. Once applied, the turn is kept as a [`Turn`]: one
//! ChangeHistoryService recording in Studio and, in a git project, one commit.

use std::fmt;

/// One change to an instance in the place.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Change {
    /// A new instance, for example `Workspace.Lobby` (a Model with 26 parts).
    Add {
        path: String,
        class: String,
        detail: String,
    },
    /// An existing instance whose properties, parent or source change.
    Modify { path: String, detail: String },
    /// An instance that will be destroyed.
    Remove { path: String, class: String },
}

impl Change {
    pub fn add(path: &str, class: &str, detail: &str) -> Self {
        Change::Add {
            path: path.into(),
            class: class.into(),
            detail: detail.into(),
        }
    }

    pub fn modify(path: &str, detail: &str) -> Self {
        Change::Modify {
            path: path.into(),
            detail: detail.into(),
        }
    }

    pub fn remove(path: &str, class: &str) -> Self {
        Change::Remove {
            path: path.into(),
            class: class.into(),
        }
    }

    /// The sign shown in the terminal: `+`, `~` or `-`.
    pub fn sign(&self) -> char {
        match self {
            Change::Add { .. } => '+',
            Change::Modify { .. } => '~',
            Change::Remove { .. } => '-',
        }
    }

    pub fn path(&self) -> &str {
        match self {
            Change::Add { path, .. }
            | Change::Modify { path, .. }
            | Change::Remove { path, .. } => path,
        }
    }
}

impl fmt::Display for Change {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Change::Add {
                path,
                class,
                detail,
            } if detail.is_empty() => {
                write!(f, "+ {path}  {class}")
            }
            Change::Add {
                path,
                class,
                detail,
            } => write!(f, "+ {path}  {class} · {detail}"),
            Change::Modify { path, detail } => write!(f, "~ {path}  {detail}"),
            Change::Remove { path, class } => write!(f, "- {path}  {class}"),
        }
    }
}

/// Everything one turn wants to do to the place.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PlaceDiff {
    pub changes: Vec<Change>,
}

impl PlaceDiff {
    pub fn push(&mut self, change: Change) -> &mut Self {
        self.changes.push(change);
        self
    }

    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    /// Counts of added, modified and removed instances.
    pub fn counts(&self) -> (usize, usize, usize) {
        self.changes.iter().fold((0, 0, 0), |(a, m, r), c| match c {
            Change::Add { .. } => (a + 1, m, r),
            Change::Modify { .. } => (a, m + 1, r),
            Change::Remove { .. } => (a, m, r + 1),
        })
    }

    /// The question asked before anything is applied.
    pub fn question(&self, place: &str) -> String {
        match self.changes.len() {
            0 => format!("Nothing to change in {place}."),
            1 => format!("Apply 1 change to {place}?"),
            n => format!("Apply {n} changes to {place}?"),
        }
    }
}

/// An applied turn, kept so it can be rewound later.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Turn {
    pub number: u32,
    pub summary: String,
    /// Name passed to `ChangeHistoryService:TryBeginRecording`.
    pub recording: String,
    /// The git commit for this turn, when the project is a git repository.
    pub commit: Option<String>,
}

impl Turn {
    pub fn new(number: u32, summary: &str) -> Self {
        Turn {
            number,
            summary: summary.into(),
            recording: format!("Baseplate turn {number}"),
            commit: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn lobby() -> PlaceDiff {
        let mut d = PlaceDiff::default();
        d.push(Change::add("Workspace.Lobby", "Model", "26 parts"))
            .push(Change::add("Workspace.Coins", "Folder", "20 parts"))
            .push(Change::add(
                "ServerScriptService.CoinService",
                "Script",
                "48 lines",
            ))
            .push(Change::add("StarterGui.CoinCounter", "ScreenGui", ""))
            .push(Change::modify(
                "Workspace.SpawnLocation",
                "into Lobby.SpawnPad",
            ));
        d
    }

    #[test]
    fn counts_and_question() {
        let d = lobby();
        assert_eq!(d.counts(), (4, 1, 0));
        assert_eq!(d.question("Coin Rush"), "Apply 5 changes to Coin Rush?");
        assert_eq!(
            PlaceDiff::default().question("Coin Rush"),
            "Nothing to change in Coin Rush."
        );
    }

    #[test]
    fn lines_read_like_the_terminal() {
        let d = lobby();
        assert_eq!(
            d.changes[0].to_string(),
            "+ Workspace.Lobby  Model · 26 parts"
        );
        assert_eq!(
            d.changes[3].to_string(),
            "+ StarterGui.CoinCounter  ScreenGui"
        );
        assert_eq!(d.changes[4].sign(), '~');
        assert_eq!(
            Change::remove("Workspace.Old", "Part").to_string(),
            "- Workspace.Old  Part"
        );
    }

    #[test]
    fn turns_name_their_recording() {
        assert_eq!(Turn::new(4, "lobby").recording, "Baseplate turn 4");
    }
}
