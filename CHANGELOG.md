# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.3] - 2024-06-30

Add a template builder.

## [0.1.2] - 2024-06-29

Little correction of all examples for the `README` so it's now possible to just copy / paste them.

## [0.1.1] - 2024-06-29

Introduction of the functions `generate_with_options` and `parse_with_options`, the structure `StringTemplaterOptions` and the type `OverrideMessage`.

With this, we'll be able to handle missing keys using the options. This might help with not wanting to crash the application in case a key is missing, but we can choose to either hide or display the missing field keys in the template itself.

If you decide to display the missing field keys, you can choose to override the message however you want.

## [0.1.0] - 2024-06-29

Initial release.
