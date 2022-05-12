# Component generator

component_generator allows you to generate custom component folder structure.

**Example:**

```sh
cd example && cargo run -- -d ./ -a -r "^[A-Z].*" -classname transform:kebab-case BeautifulComponent
```

The above command will generate the following structure:

    .
    ├── example
    │   ├── BeautifulComponent
    │   │   ├── BeautifulComponent.js
    │   │   ├── BeautifulComponent.scss
    │   │   ├── BeautifulComponent.spec.js
    │   │   ├── BeautifulComponent.stories.js
    │   │   ├── BeautifulComponentPlaceholder.js
    │   │   ├── BeautifulComponentPlaceholder.scss
    │   │   └── index.js
    │   └──
    └── ...

## Synopsis

```sh
cargo run -- -d DESTINATION_PATH [OPTIONS] COMPONENT_NAME
```

## About

**component_generator** is a CLI utility that generates files based on the provided template within the `cg_template` folder.

## Options

| Option | Value  | Description                                                                                                                                                                                                                                                                                                                                                  |
| ------ | ------ | ------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| -d     | string | **REQUIRED** Destination directory path.                                                                                                                                                                                                                                                                                                                     |
| -r     | regex  | Regex for component name validation.                                                                                                                                                                                                                                                                                                                         |
| -a     |        | Generate all the files within the template folder.                                                                                                                                                                                                                                                                                                           |
| -any   | string | Custom option that will be used to replace `{{any}}` within the template files. There is a custom value of `transform:kebab-case` that will transform the component name to kebab-case and use it to replace `{{any}}`. Example `-classname transform:kebab-case` will transform the component name to kebab case and use it to replace the `{{classname}}`. |
