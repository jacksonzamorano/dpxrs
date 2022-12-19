# DPX
Dropbox API from the command line.

## Install
Install by running `cargo install dpx`

## Usage
### Download a file
`dpx dl dropbox_file_path`.

Will download to current directory.
### Upload a file
`dpx up local_file_path`

Will upload a file at the path provided. If the path is absolute, the file will be uploaded to the Dropbox root. If relative, the relative path will be the location in Dropbox as well.
### Download a folder
`dpx folder dropbox_folder_path`

Will download a whole folder, zipped, then unzip in place.
### Download a ZIP
`dpx zip dropbox_file_or_folder_path`

Will download any folder or file in a zipped format.
### Get view-only link
`dpx vl dropbox_file_or_folder_path`

Returns a view-only link to a Dropbox file or folder.
### Remove links
`dpx private dropbox_file_or_folder_path`

Removes all Dropbox links but keeps collaborators.
### Sign out
`dpx logout`

Removes your stored Dropbox token.
### View logged in use.
`dpx whoami`

Shows the signed-in user's name.