# changelog
Handle your application versions history.

## Commands

### `changelog init`
Initialize changelog. Creates `data.json` file, `.gitignore` file.

### `changelog get`
Returns current list of changes in format:<br>
```
- Item
- Item 2
```

### `changelog new “Text”`
Sets new change to list.
Returns current list of changes in format:
```
— Item
— Item 2
— Text
```

### `changelog remove`
Removes change in current list.
Returns console select to choose item to remove.
After selecting returns current list of changes in format:
```
— Item
— Item 2
```

### `changelog remove 0` 
Removes change in current list by it’s index.
Returns current list of changes in format:
```
— Item
— Item 2
```

### `changelog build`
Builds version log.
Returns console select to choose version to build.
Returns chosen build list of changes in format:
```
— Item
— Item 2
```

### `changelog build "0.02"`
Builds specific version log.
Returns chosen build list of changes in format:
```
— Item
— Item 2
```

### `changelog build --current`
Builds current version log.
Returns chosen build list of changes in format:
```
— Item
— Item 2
```

### `changelog release`
Releases current version log.
Automatically increments version of application.
Builds current changes list into `/releases` folder.
Puts current changes into archive and clears current changes.
Returns released list of changes in format:
```
— Item
— Item 2
```