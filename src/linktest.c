#include <stdio.h>
#include <stddef.h>
#include <stdbool.h>

#ifdef _WIN32

  /* You should define ADD_EXPORTS *only* when building the DLL. */
  #ifdef ADD_EXPORTS
    #define ADDAPI __declspec(dllexport)
    #define ADDCALL __cdecl
  #else
    #define ADDAPI
    #define ADDCALL
  #endif

#else /* _WIN32 not defined. */

  /* Define with no value on non-Windows OSes. */
  #define ADDAPI
  #define ADDCALL

#endif



// Some simple function signatures from libsass.
struct Sass_Options; // base struct

ADDAPI struct Sass_Options* ADDCALL sass_make_options (void);
ADDAPI void ADDCALL sass_delete_options(struct Sass_Options* options);
ADDAPI void ADDCALL sass_option_set_precision (struct Sass_Options* options, int precision);


// .a is a static library
// .so is a dynamic shared library (a DLL).
//
// To compile against the system-installed library:
//    $ gcc linktest.c -lsass
// To compile against my library (pthread and dl are extra libs we need to link to avoid undefined symbols):
//    $ gcc -o linktest linktest.c -L../target/debug -lsassafras -lpthread -ldl

int main() {
    struct Sass_Options* opt = sass_make_options();
    printf("sass_make_options ok\n");

    sass_option_set_precision(opt, 13);
    printf("sass_option_set_precision ok\n");

    // symbol is not defined for some reason in the apt-getted library. Looks like a Mint error:
    // https://forums.linuxmint.com/viewtopic.php?f=47&t=268871
    //sass_delete_options(opt);
    //printf("sass_delete_options ok\n");

    return 0;
}
