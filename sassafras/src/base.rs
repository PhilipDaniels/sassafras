/*
// to allocate buffer to be filled
ADDAPI void* ADDCALL sass_alloc_memory(size_t size);
// to allocate a buffer from existing string
ADDAPI char* ADDCALL sass_copy_c_string(const char* str);
// to free overtaken memory when done
ADDAPI void ADDCALL sass_free_memory(void* ptr);

// Some convenient string helper function
ADDAPI char* ADDCALL sass_string_quote (const char* str, const char quote_mark);
ADDAPI char* ADDCALL sass_string_unquote (const char* str);

// Implemented sass language version
// Hardcoded version 3.4 for time being
ADDAPI const char* ADDCALL libsass_version(void);

// Get compiled libsass language
ADDAPI const char* ADDCALL libsass_language_version(void);
*/