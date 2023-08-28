use ffi;
use std::error;
use std::fmt;

pub type FtResult<T> = Result<T, Error>;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
#[repr(i32)]
pub enum Error {
    Ok = ffi::FT_Err_Ok,
    CannotOpenResource = ffi::FT_Err_Cannot_Open_Resource,
    UnknownFileFormat = ffi::FT_Err_Unknown_File_Format,
    InvalidFileFormat = ffi::FT_Err_Invalid_File_Format,
    InvalidVersion = ffi::FT_Err_Invalid_Version,
    LowerModuleVersion = ffi::FT_Err_Lower_Module_Version,
    InvalidArgument = ffi::FT_Err_Invalid_Argument,
    UnimplementedFeature = ffi::FT_Err_Unimplemented_Feature,
    InvalidTable = ffi::FT_Err_Invalid_Table,
    InvalidOffset = ffi::FT_Err_Invalid_Offset,
    ArrayTooLarge = ffi::FT_Err_Array_Too_Large,
    MissingModule = ffi::FT_Err_Missing_Module,
    MissingProperty = ffi::FT_Err_Missing_Property,
    InvalidGlyphIndex = ffi::FT_Err_Invalid_Glyph_Index,
    InvalidCharacterCode = ffi::FT_Err_Invalid_Character_Code,
    InvalidGlyphFormat = ffi::FT_Err_Invalid_Glyph_Format,
    CannotRenderGlyph = ffi::FT_Err_Cannot_Render_Glyph,
    InvalidOutline = ffi::FT_Err_Invalid_Outline,
    InvalidComposite = ffi::FT_Err_Invalid_Composite,
    TooManyHints = ffi::FT_Err_Too_Many_Hints,
    InvalidPixelSize = ffi::FT_Err_Invalid_Pixel_Size,
    InvalidHandle = ffi::FT_Err_Invalid_Handle,
    InvalidLibraryHandle = ffi::FT_Err_Invalid_Library_Handle,
    InvalidDriverHandle = ffi::FT_Err_Invalid_Driver_Handle,
    InvalidFaceHandle = ffi::FT_Err_Invalid_Face_Handle,
    InvalidSizeHandle = ffi::FT_Err_Invalid_Size_Handle,
    InvalidSlotHandle = ffi::FT_Err_Invalid_Slot_Handle,
    InvalidCharMapHandle = ffi::FT_Err_Invalid_CharMap_Handle,
    InvalidCacheHandle = ffi::FT_Err_Invalid_Cache_Handle,
    InvalidStreamHandle = ffi::FT_Err_Invalid_Stream_Handle,
    TooManyDrivers = ffi::FT_Err_Too_Many_Drivers,
    TooManyExtensions = ffi::FT_Err_Too_Many_Extensions,
    OutOfMemory = ffi::FT_Err_Out_Of_Memory,
    UnlistedObject = ffi::FT_Err_Unlisted_Object,
    CannotOpenStream = ffi::FT_Err_Cannot_Open_Stream,
    InvalidStreamSeek = ffi::FT_Err_Invalid_Stream_Seek,
    InvalidStreamSkip = ffi::FT_Err_Invalid_Stream_Skip,
    InvalidStreamRead = ffi::FT_Err_Invalid_Stream_Read,
    InvalidStreamOperation = ffi::FT_Err_Invalid_Stream_Operation,
    InvalidFrameOperation = ffi::FT_Err_Invalid_Frame_Operation,
    NestedFrameAccess = ffi::FT_Err_Nested_Frame_Access,
    InvalidFrameRead = ffi::FT_Err_Invalid_Frame_Read,
    RasterUninitialized = ffi::FT_Err_Raster_Uninitialized,
    RasterCorrupted = ffi::FT_Err_Raster_Corrupted,
    RasterOverflow = ffi::FT_Err_Raster_Overflow,
    RasterNegativeHeight = ffi::FT_Err_Raster_Negative_Height,
    TooManyCaches = ffi::FT_Err_Too_Many_Caches,
    InvalidOpcode = ffi::FT_Err_Invalid_Opcode,
    TooFewArguments = ffi::FT_Err_Too_Few_Arguments,
    StackOverflow = ffi::FT_Err_Stack_Overflow,
    CodeOverflow = ffi::FT_Err_Code_Overflow,
    BadArgument = ffi::FT_Err_Bad_Argument,
    DivideByZero = ffi::FT_Err_Divide_By_Zero,
    InvalidReference = ffi::FT_Err_Invalid_Reference,
    DebugOpCode = ffi::FT_Err_Debug_OpCode,
    ENDFInExecStream = ffi::FT_Err_ENDF_In_Exec_Stream,
    NestedDEFS = ffi::FT_Err_Nested_DEFS,
    InvalidCodeRange = ffi::FT_Err_Invalid_CodeRange,
    ExecutionTooLong = ffi::FT_Err_Execution_Too_Long,
    TooManyFunctionDefs = ffi::FT_Err_Too_Many_Function_Defs,
    TooManyInstructionDefs = ffi::FT_Err_Too_Many_Instruction_Defs,
    TableMissing = ffi::FT_Err_Table_Missing,
    HorizHeaderMissing = ffi::FT_Err_Horiz_Header_Missing,
    LocationsMissing = ffi::FT_Err_Locations_Missing,
    NameTableMissing = ffi::FT_Err_Name_Table_Missing,
    CMapTableMissing = ffi::FT_Err_CMap_Table_Missing,
    HmtxTableMissing = ffi::FT_Err_Hmtx_Table_Missing,
    PostTableMissing = ffi::FT_Err_Post_Table_Missing,
    InvalidHorizMetrics = ffi::FT_Err_Invalid_Horiz_Metrics,
    InvalidCharMapFormat = ffi::FT_Err_Invalid_CharMap_Format,
    InvalidPPem = ffi::FT_Err_Invalid_PPem,
    InvalidVertMetrics = ffi::FT_Err_Invalid_Vert_Metrics,
    CouldNotFindContext = ffi::FT_Err_Could_Not_Find_Context,
    InvalidPostTableFormat = ffi::FT_Err_Invalid_Post_Table_Format,
    InvalidPostTable = ffi::FT_Err_Invalid_Post_Table,
    Syntax = ffi::FT_Err_Syntax_Error,
    StackUnderflow = ffi::FT_Err_Stack_Underflow,
    Ignore = ffi::FT_Err_Ignore,
    NoUnicodeGlyphName = ffi::FT_Err_No_Unicode_Glyph_Name,
    MissingStartfontField = ffi::FT_Err_Missing_Startfont_Field,
    MissingFontField = ffi::FT_Err_Missing_Font_Field,
    MissingSizeField = ffi::FT_Err_Missing_Size_Field,
    MissingFontboundingboxField = ffi::FT_Err_Missing_Fontboundingbox_Field,
    MissingCharsField = ffi::FT_Err_Missing_Chars_Field,
    MissingStartcharField = ffi::FT_Err_Missing_Startchar_Field,
    MissingEncodingField = ffi::FT_Err_Missing_Encoding_Field,
    MissingBbxField = ffi::FT_Err_Missing_Bbx_Field,
    BbxTooBig = ffi::FT_Err_Bbx_Too_Big,
    CorruptedFontHeader = ffi::FT_Err_Corrupted_Font_Header,
    CorruptedFontGlyphs = ffi::FT_Err_Corrupted_Font_Glyphs,
    Max = ffi::FT_Err_Max,
    UnexpectedPixelMode,
    InvalidPath,
    Unknown,
}

impl From<i32> for Error {
    fn from(err: i32) -> Self {
        match err {
            ffi::FT_Err_Ok => Error::Ok,
            ffi::FT_Err_Cannot_Open_Resource => Error::CannotOpenResource,
            ffi::FT_Err_Unknown_File_Format => Error::UnknownFileFormat,
            ffi::FT_Err_Invalid_File_Format => Error::InvalidFileFormat,
            ffi::FT_Err_Invalid_Version => Error::InvalidVersion,
            ffi::FT_Err_Lower_Module_Version => Error::LowerModuleVersion,
            ffi::FT_Err_Invalid_Argument => Error::InvalidArgument,
            ffi::FT_Err_Unimplemented_Feature => Error::UnimplementedFeature,
            ffi::FT_Err_Invalid_Table => Error::InvalidTable,
            ffi::FT_Err_Invalid_Offset => Error::InvalidOffset,
            ffi::FT_Err_Array_Too_Large => Error::ArrayTooLarge,
            ffi::FT_Err_Missing_Module => Error::MissingModule,
            ffi::FT_Err_Missing_Property => Error::MissingProperty,
            ffi::FT_Err_Invalid_Glyph_Index => Error::InvalidGlyphIndex,
            ffi::FT_Err_Invalid_Character_Code => Error::InvalidCharacterCode,
            ffi::FT_Err_Invalid_Glyph_Format => Error::InvalidGlyphFormat,
            ffi::FT_Err_Cannot_Render_Glyph => Error::CannotRenderGlyph,
            ffi::FT_Err_Invalid_Outline => Error::InvalidOutline,
            ffi::FT_Err_Invalid_Composite => Error::InvalidComposite,
            ffi::FT_Err_Too_Many_Hints => Error::TooManyHints,
            ffi::FT_Err_Invalid_Pixel_Size => Error::InvalidPixelSize,
            ffi::FT_Err_Invalid_Handle => Error::InvalidHandle,
            ffi::FT_Err_Invalid_Library_Handle => Error::InvalidLibraryHandle,
            ffi::FT_Err_Invalid_Driver_Handle => Error::InvalidDriverHandle,
            ffi::FT_Err_Invalid_Face_Handle => Error::InvalidFaceHandle,
            ffi::FT_Err_Invalid_Size_Handle => Error::InvalidSizeHandle,
            ffi::FT_Err_Invalid_Slot_Handle => Error::InvalidSlotHandle,
            ffi::FT_Err_Invalid_CharMap_Handle => Error::InvalidCharMapHandle,
            ffi::FT_Err_Invalid_Cache_Handle => Error::InvalidCacheHandle,
            ffi::FT_Err_Invalid_Stream_Handle => Error::InvalidStreamHandle,
            ffi::FT_Err_Too_Many_Drivers => Error::TooManyDrivers,
            ffi::FT_Err_Too_Many_Extensions => Error::TooManyExtensions,
            ffi::FT_Err_Out_Of_Memory => Error::OutOfMemory,
            ffi::FT_Err_Unlisted_Object => Error::UnlistedObject,
            ffi::FT_Err_Cannot_Open_Stream => Error::CannotOpenStream,
            ffi::FT_Err_Invalid_Stream_Seek => Error::InvalidStreamSeek,
            ffi::FT_Err_Invalid_Stream_Skip => Error::InvalidStreamSkip,
            ffi::FT_Err_Invalid_Stream_Read => Error::InvalidStreamRead,
            ffi::FT_Err_Invalid_Stream_Operation => Error::InvalidStreamOperation,
            ffi::FT_Err_Invalid_Frame_Operation => Error::InvalidFrameOperation,
            ffi::FT_Err_Nested_Frame_Access => Error::NestedFrameAccess,
            ffi::FT_Err_Invalid_Frame_Read => Error::InvalidFrameRead,
            ffi::FT_Err_Raster_Uninitialized => Error::RasterUninitialized,
            ffi::FT_Err_Raster_Corrupted => Error::RasterCorrupted,
            ffi::FT_Err_Raster_Overflow => Error::RasterOverflow,
            ffi::FT_Err_Raster_Negative_Height => Error::RasterNegativeHeight,
            ffi::FT_Err_Too_Many_Caches => Error::TooManyCaches,
            ffi::FT_Err_Invalid_Opcode => Error::InvalidOpcode,
            ffi::FT_Err_Too_Few_Arguments => Error::TooFewArguments,
            ffi::FT_Err_Stack_Overflow => Error::StackOverflow,
            ffi::FT_Err_Code_Overflow => Error::CodeOverflow,
            ffi::FT_Err_Bad_Argument => Error::BadArgument,
            ffi::FT_Err_Divide_By_Zero => Error::DivideByZero,
            ffi::FT_Err_Invalid_Reference => Error::InvalidReference,
            ffi::FT_Err_Debug_OpCode => Error::DebugOpCode,
            ffi::FT_Err_ENDF_In_Exec_Stream => Error::ENDFInExecStream,
            ffi::FT_Err_Nested_DEFS => Error::NestedDEFS,
            ffi::FT_Err_Invalid_CodeRange => Error::InvalidCodeRange,
            ffi::FT_Err_Execution_Too_Long => Error::ExecutionTooLong,
            ffi::FT_Err_Too_Many_Function_Defs => Error::TooManyFunctionDefs,
            ffi::FT_Err_Too_Many_Instruction_Defs => Error::TooManyInstructionDefs,
            ffi::FT_Err_Table_Missing => Error::TableMissing,
            ffi::FT_Err_Horiz_Header_Missing => Error::HorizHeaderMissing,
            ffi::FT_Err_Locations_Missing => Error::LocationsMissing,
            ffi::FT_Err_Name_Table_Missing => Error::NameTableMissing,
            ffi::FT_Err_CMap_Table_Missing => Error::CMapTableMissing,
            ffi::FT_Err_Hmtx_Table_Missing => Error::HmtxTableMissing,
            ffi::FT_Err_Post_Table_Missing => Error::PostTableMissing,
            ffi::FT_Err_Invalid_Horiz_Metrics => Error::InvalidHorizMetrics,
            ffi::FT_Err_Invalid_CharMap_Format => Error::InvalidCharMapFormat,
            ffi::FT_Err_Invalid_PPem => Error::InvalidPPem,
            ffi::FT_Err_Invalid_Vert_Metrics => Error::InvalidVertMetrics,
            ffi::FT_Err_Could_Not_Find_Context => Error::CouldNotFindContext,
            ffi::FT_Err_Invalid_Post_Table_Format => Error::InvalidPostTableFormat,
            ffi::FT_Err_Invalid_Post_Table => Error::InvalidPostTable,
            ffi::FT_Err_Syntax_Error => Error::Syntax,
            ffi::FT_Err_Stack_Underflow => Error::StackUnderflow,
            ffi::FT_Err_Ignore => Error::Ignore,
            ffi::FT_Err_No_Unicode_Glyph_Name => Error::NoUnicodeGlyphName,
            ffi::FT_Err_Missing_Startfont_Field => Error::MissingStartfontField,
            ffi::FT_Err_Missing_Font_Field => Error::MissingFontField,
            ffi::FT_Err_Missing_Size_Field => Error::MissingSizeField,
            ffi::FT_Err_Missing_Fontboundingbox_Field => Error::MissingFontboundingboxField,
            ffi::FT_Err_Missing_Chars_Field => Error::MissingCharsField,
            ffi::FT_Err_Missing_Startchar_Field => Error::MissingStartcharField,
            ffi::FT_Err_Missing_Encoding_Field => Error::MissingEncodingField,
            ffi::FT_Err_Missing_Bbx_Field => Error::MissingBbxField,
            ffi::FT_Err_Bbx_Too_Big => Error::BbxTooBig,
            ffi::FT_Err_Corrupted_Font_Header => Error::CorruptedFontHeader,
            ffi::FT_Err_Corrupted_Font_Glyphs => Error::CorruptedFontGlyphs,
            ffi::FT_Err_Max => Error::Max,
            _ => Error::Unknown,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use self::Error::*;
        f.write_str(match *self {
            Ok => "Ok",
            CannotOpenResource => "Cannot open resource",
            UnknownFileFormat => "Unknown file format",
            InvalidFileFormat => "Invalid file format",
            InvalidVersion => "Invalid version",
            LowerModuleVersion => "Lower module version",
            InvalidArgument => "Invalid argument",
            UnimplementedFeature => "Unimplemented feature",
            InvalidTable => "Invalid table",
            InvalidOffset => "Invalid offset",
            ArrayTooLarge => "Array too large",
            MissingModule => "Missing module",
            MissingProperty => "Missing property",
            InvalidGlyphIndex => "Invalid glyph index",
            InvalidCharacterCode => "Invalid character code",
            InvalidGlyphFormat => "Invalid glyph format",
            CannotRenderGlyph => "Cannot render glyph",
            InvalidOutline => "Invalid outline",
            InvalidComposite => "Invalid composite",
            TooManyHints => "Too many hints",
            InvalidPixelSize => "Invalid pixel size",
            InvalidHandle => "Invalid handle",
            InvalidLibraryHandle => "Invalid library handle",
            InvalidDriverHandle => "Invalid driver handle",
            InvalidFaceHandle => "Invalid face handle",
            InvalidSizeHandle => "Invalid size handle",
            InvalidSlotHandle => "Invalid slot handle",
            InvalidCharMapHandle => "Invalid char map handle",
            InvalidCacheHandle => "Invalid cache handle",
            InvalidStreamHandle => "Invalid stream handle",
            TooManyDrivers => "Too many drivers",
            TooManyExtensions => "Too many extensions",
            OutOfMemory => "Out of memory",
            UnlistedObject => "Unlisted object",
            CannotOpenStream => "Cannot open stream",
            InvalidStreamSeek => "Invalid stream seek",
            InvalidStreamSkip => "Invalid stream skip",
            InvalidStreamRead => "Invalid stream read",
            InvalidStreamOperation => "Invalid stream operation",
            InvalidFrameOperation => "Invalid frame operation",
            NestedFrameAccess => "Nested frame access",
            InvalidFrameRead => "Invalid frame read",
            RasterUninitialized => "Raster uninitialized",
            RasterCorrupted => "Raster corrupted",
            RasterOverflow => "Raster overflow",
            RasterNegativeHeight => "Raster negative height",
            TooManyCaches => "Too many caches",
            InvalidOpcode => "Invalid opcode",
            TooFewArguments => "Too few arguments",
            StackOverflow => "Stack overflow",
            CodeOverflow => "Code overflow",
            BadArgument => "Bad argument",
            DivideByZero => "Divide by zero",
            InvalidReference => "Invalid reference",
            DebugOpCode => "Debug op code",
            ENDFInExecStream => "ENDF in exec stream",
            NestedDEFS => "Nested DEFS",
            InvalidCodeRange => "Invalid code range",
            ExecutionTooLong => "Execution too long",
            TooManyFunctionDefs => "Too many function defs",
            TooManyInstructionDefs => "Too many instruction defs",
            TableMissing => "Table missing",
            HorizHeaderMissing => "Horiz header missing",
            LocationsMissing => "Locations missing",
            NameTableMissing => "Name table missing",
            CMapTableMissing => "C map table missing",
            HmtxTableMissing => "Hmtx table missing",
            PostTableMissing => "Post table missing",
            InvalidHorizMetrics => "Invalid horiz metrics",
            InvalidCharMapFormat => "Invalid char map format",
            InvalidPPem => "Invalid p pem",
            InvalidVertMetrics => "Invalid vert metrics",
            CouldNotFindContext => "Could not find context",
            InvalidPostTableFormat => "Invalid post table format",
            InvalidPostTable => "Invalid post table",
            Syntax => "Syntax",
            StackUnderflow => "Stack underflow",
            Ignore => "Ignore",
            NoUnicodeGlyphName => "No unicode glyph name",
            MissingStartfontField => "Missing startfont field",
            MissingFontField => "Missing font field",
            MissingSizeField => "Missing size field",
            MissingFontboundingboxField => "Missing fontboundingbox field",
            MissingCharsField => "Missing chars field",
            MissingStartcharField => "Missing startchar field",
            MissingEncodingField => "Missing encoding field",
            MissingBbxField => "Missing bbx field",
            BbxTooBig => "Bbx too big",
            CorruptedFontHeader => "Corrupted font header",
            CorruptedFontGlyphs => "Corrupted font glyphs",
            Max => "Max",
            UnexpectedPixelMode => "Unexpected pixel mode",
            InvalidPath => "Invalid path",
            Unknown => "Unknown",
        })
    }
}

impl error::Error for Error {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn print_should_halt() {
        use std::error::Error as _;

        Error::Ok.to_string();
        #[allow(deprecated)]
        Error::Ok.description();
    }
}
