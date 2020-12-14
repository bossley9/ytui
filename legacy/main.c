#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <curl/curl.h>

/* typedef struct _string { */
/*   char* str; */
/*   unsigned int len; */
/* } string; */

/* ansi color codes */
const char* RED = "\033[31m";
const char* NC = "\033[0m";

unsigned int getLength(char* str) {
  /* manual strlen for practice */
  unsigned int c = 0;
  while (str[c] != '\0') c++;
  return c;
}

/* getSearchStr makes the assumption that a search */
/* string exists to begin with */
char* getSearchStr(int argc, char* argv[]) {
  /* remove binary name from args */
  char** argArr = argv + sizeof(char);
  int argNum = argc - 1;

  for (unsigned int i = 0; i < argNum - 1; i++) {
    unsigned int c = getLength(argArr[i]);

    /* replace NUL characters in between each string */
    /* with a SPACE character (effectively joining all */
    /* arguments with spaces) */
    argArr[i][c] = ' ';
  }

  return argArr[0];
}

char* urlEncode(char* url) {
  /* at a maximum, every character could be converted */
  /* to a three character string */
  unsigned int bufferLen = getLength(url) * 3;
  char* buffer = (char*) malloc(sizeof(char) * bufferLen);
  /* buffer char counter */
  unsigned int bc = 0;
  /* url char counter */
  unsigned int c = 0;

  while (url[c] != '\0') {
    /* printf("char is %c\n", url[c]); */

    switch (url[c]) {
      case ' ': {
        *(buffer+bc) = '%';
        bc++;
        *(buffer+bc) = '2';
        bc++;
        *(buffer+bc) = '0';
        break;
      }
      case '\'': {
        *(buffer+bc) = '%';
        bc++;
        *(buffer+bc) = '9';
        bc++;
        *(buffer+bc) = '1';
        break;
      }
      case '%': {
        *(buffer+bc) = '%';
        bc++;
        *(buffer+bc) = '2';
        bc++;
        *(buffer+bc) = '5';
        break;
      }
      default: {
        *(buffer+bc) = url[c];
      }
    }

    bc++;
    c++;
  }

  return buffer;
}

size_t writeCallback(char* ptr, size_t size, size_t nmemb, void* userdata) {
  char* str = (char*) userdata;
  unsigned int len = getLength(str);
  unsigned int newLen = len + size*nmemb;

  /* printf("data so far is %s\n", str); */
  /* printf("len is %i\n", len); */

  /* str = (char*) realloc(str, newLen + 1); */
  /* memcpy(str+len, ptr, size+nmemb); */
  /* *(str + newLen) = '\0'; */

  /* strcat(userdata, ptr); */

  /* printf("bytes are %s\n", ptr); */

  return size*nmemb;
}

int main(int argc, char* argv[]) {
  if (argc == 1) {
    printf("%sUSAGE: ytui [search query]%s\n", RED, NC);
    return 1;
  }

  char* searchStr = urlEncode(getSearchStr(argc, argv));
  const char* FQDN = "https://www.youtube.com";
  const char* RESOURCE_SEARCH_RESULTS = "/results?search_query=";

  unsigned int searchStrLen = getLength(searchStr);
  unsigned int domainLen = getLength((char*) FQDN);
  unsigned int resourceLen = getLength((char*) RESOURCE_SEARCH_RESULTS);

  char* searchUrl = (char*) malloc(sizeof(char)* (searchStrLen + domainLen + resourceLen));

  strcat(searchUrl, FQDN);
  strcat(searchUrl, RESOURCE_SEARCH_RESULTS);
  strcat(searchUrl, searchStr);

  free(searchStr);

  /* printf("url is %s\n", searchUrl); */

  /* TODO */
  /* use multi for async calls */

  CURL* curl = curl_easy_init();
  if (!curl) {
    printf("%sERROR: Unable to use libcurl. Make sure curl is installed.%s\n", RED, NC);
    return 1;
  }

  /* char* data = malloc(sizeof(char) * 20 * 1024); */
  char* data = malloc(sizeof(char));
  *data = '\0';

  curl_easy_setopt(curl, CURLOPT_URL, searchUrl);
  curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, writeCallback);
  curl_easy_setopt(curl, CURLOPT_WRITEDATA, data);

  /* Perform the request, res will get the return code */
  CURLcode res = curl_easy_perform(curl);

  printf("BEGIN\n%s\nEOF\n", data);
  /* Check for errors */
  if (res != CURLE_OK) {
    printf("%sERROR: Status code %i%s\n", RED, res, NC);
    printf("%sERROR: Unable to perform get request. Make sure curl is installed.%s\n", RED, NC);
    return 1;
  }

  /* cleanup */
  curl_easy_cleanup(curl);

  free(searchUrl);
  free(data);

  return 0;
}
