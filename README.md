# procelio-localization
## The Rust executable is used for generating and compiling localization files. It *must* be ran from the command line.

The available commands:
#### create: `executableName.exe create LANGUAGE path/to/files/directory`
Generates the files for the language of the given name by creating a subdirectory in the files folder.
- If the language does not exist already, creates new files
- If the language already exists, updates its files to contain the current set of UI fields (as defined in the enum file)

#### validate: `executableName.exe validate LANGUAGE path/to/files/directory`
Validates that a localization file is suitable for use in-game. Checks that
- The JSON file can be properly parsed
- No fields are defined multiple times
- All fields are defined
- All fields are present in the enum file

#### generate: `executableName.exe generate path/to/files/directory`
Generates all localization languages declared in the files/config.json file
Stores the output files in the output folder (defined in config)


## Customizing Localization Files
For saving vertical space, only the field_name and field_value values are generated by default.
If further alternations are required, various parameters of the text can be overwritten by adding the corresponding flag.

Here is a fully-specified text element
```
{
      "field_name": "SOME_TEXT_FIELD",
      "field_value": "Some Text Value In Game",
      "text_size": 0,
      "bold": false,
      "italic": false,
      "underline": false,
      "strikethrough": false,
      "text_color": [
        255,
        255,
        255
      ]
}
```
Descriptions of each field are as follows:

#### field_name
The internal name of the field, used so the game can tell which UI element this goes with.
DO NOT CHANGE

#### field_value
The text of the UI element. You almost certainly want to change this.

#### text_size
In case it's necessary to overwrite the size of the text. 0 = default
If unspecified, treated as 0

#### bold
Whether the field value should be bold in-game
If unspecified, treated as 'false'

#### italic
Whether the field value should be italic in-game
If unspecified, treated as 'false'

#### underline
Whether the field value should be underlined in game
If unspecified, treated as 'false'

#### strikethrough
Whether the field value should be struck through in-game
If unspecified, treated as 'false'

#### text_color
What color the text should be in-game
If unspecified, treated as white '[255, 255, 255]'. Each number is the respective one-byte R,G,B channel


### License
This project is made available under the Apache License 2.0.