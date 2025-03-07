# spotify-party

This project is an attempt to let a crowd interact with a Spotify session hosted on a single account in the sense of dynamically adding songs to the current queue

This Repository is a work in progress, use it at your own risk.
The initial version is a stupid attempt using neither websockets nor services in the Frontend (because why would you anyways).
A rewrite of the backend (and propably also the frontend) is pending

# Workflow

- One User connects their Spotify account to the Backend via the Login
- All users having access to the Frontend are free to search for songs available on Spotify and suggest them
- Suggested songs can be upvoted by all participants
- Every 2 minutes the most upvoted song at that moment is added to the queue
