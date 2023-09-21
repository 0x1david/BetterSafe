# BetterSafe
BetterSafe is a CLI tool developed to protect you from making dumb, irrevertable changes with a single command.

It does this by archiving files you remove for a certain time, the more dangerous the action, the longer time to restore it from the archives.

By default arguments are similar to classic bash rm command - *Force*, *Interactive*, *Recursive*, *Dir*, *Verbose*

### Additional Options

**Trash (path)** - Skips Archiving and deletes the file immediately

**Abandon /(archived path)** - Permanently removes all files in the whole archive (or a given archived path) 

**Restore /(archived path)** - Restores all files in the whole archive (or a given archived path)

**Archive (path)** - Archives all files in a given path indefinitely
