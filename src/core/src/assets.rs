///
/// Define the main application name (can we store that elsewhere ?)
///
pub const PROJECT_NAME: &str = "PhotosNorm";

pub const DOC_CLI: &str = "PhotosNorm: A simple tool to lossless manipulate images properties.\n\
                           \n\
                           info: display some EXIF info.\n\
                           set:  Update some EXIF tags. More info below or with set --help.\n\
                           fix:  Fix properties like orientation, file name, ... More info below or with fix --help.\n\
                           \n\
                           To each command, you can provide one or more files and/or folders.\n\
                           Each known files (aka images) will be processed, other ones will be ignored.\n\
                           For each folder, all files within will be analysed like described just before. Sub-folders will be \
                           ignored (this is non-recursive).";

pub const DOC_GUI: &str = "PhotosNorm: A simple tool to lossless manipulate images properties.\n\
                           \n\
                           The GUI will display images and allows to modify some EXIF tags.\n\
                           It will also allows to fix properties like orientation, file name, ...\n\
                           \n\
                           You can provide one or more files and/or folders.\n\
                           Each known files (aka images) will be processed, other ones will be ignored.\n\
                           For each folder, all files within will be analysed like described just before. Sub-folders will be \
                           ignored (this is non-recursive).";
