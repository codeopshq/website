pub struct TutorialEntry<'a> {
    pub title: &'a str,
    pub image_url: &'a str,
    pub description: &'a str,
    pub video_url: &'a str,
    pub tags: [(&'a str, &'a str); 3],
}

pub fn get_tutorials() -> [TutorialEntry<'static>; 5] {
    [
        TutorialEntry {
            title: "Advanced Tmux",
            image_url: "https://img.youtube.com/vi/F2X9jjCT17E/mqdefault.jpg",
            description: "Upgrade your terminal game with TMUX TPM and essential plugins.",
            video_url: "https://www.youtube.com/watch?v=F2X9jjCT17E",
            tags: [
                ("#tmux", "bg-red-600"),
                ("#tpm", "bg-blue-600"),
                ("#plugins", "bg-green-600"),
            ],
        },
        TutorialEntry {
            title: "BTOP",
            image_url: "https://img.youtube.com/vi/SdBC3l6bd5k/mqdefault.jpg",
            description: "The Ultimate Linux System Monitor That's Easy To Use.",
            video_url: "https://www.youtube.com/watch?v=SdBC3l6bd5k",
            tags: [
                ("#linux", "bg-green-600"),
                ("#monitor", "bg-blue-600"),
                ("#btop", "bg-yellow-600"),
            ],
        },
        TutorialEntry {
            title: "FZF",
            image_url: "https://img.youtube.com/vi/lQzCtQzvYPU/mqdefault.jpg",
            description: "The Unbeatable Command-Line Fuzzy Finder.",
            video_url: "https://www.youtube.com/watch?v=lQzCtQzvYPU",
            tags: [
                ("#fzf", "bg-blue-600"),
                ("#terminal", "bg-red-600"),
                ("#tools", "bg-green-600"),
            ],
        },
        TutorialEntry {
            title: "TMUX in 5 Minutes",
            image_url: "https://img.youtube.com/vi/4_uwuPem9Gs/mqdefault.jpg",
            description: "Learn how to use TMUX in 5 minutes. TMUX is a terminal multiplexer.",
            video_url: "https://www.youtube.com/watch?v=4_uwuPem9Gs",
            tags: [
                ("#tmux", "bg-red-600"),
                ("#terminal", "bg-blue-600"),
                ("#tools", "bg-green-600"),
            ],
        },
        TutorialEntry {
            title: "LazyGit",
            image_url: "https://img.youtube.com/vi/dSWJKcEiAaM/mqdefault.jpg",
            description: "A simple and fast way to manage your Git repositories.",
            video_url: "https://www.youtube.com/watch?v=dSWJKcEiAaM",
            tags: [
                ("#git", "bg-green-600"),
                ("#terminal", "bg-red-600"),
                ("#tools", "bg-black"),
            ],
        },
    ]
}
