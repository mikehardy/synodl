#include <stdlib.h>
#include <string.h>

#include <curl/curl.h>

#include "syno.h"

static size_t
curl_recv(void *ptr, size_t size, size_t nmemb, struct string *s)
{
	int pos;

	pos = s->size - 1;
	s->size += (size * nmemb);
	s->ptr = realloc(s->ptr, s->size);

	if (!s->ptr)
	{
		fprintf(stderr, "Realloc failed");
		return -1;
	}

	memcpy(s->ptr + pos, ptr, size * nmemb);
	s->ptr[s->size - 1] = 0;
	return size * nmemb;
}

static void
curl_set_ssl_noverify(CURL *curl)
{
	curl_easy_setopt(curl, CURLOPT_SSL_VERIFYPEER, 0L);
	curl_easy_setopt(curl, CURLOPT_SSL_VERIFYHOST, 0L);
}

static void
curl_set_ssl_verify(CURL *curl, const char *c)
{
	curl_easy_setopt(curl, CURLOPT_CAINFO, c);
	curl_easy_setopt(curl, CURLOPT_SSL_VERIFYPEER, 1L);
}

int
curl_do(const char *url, struct cfg *cfg, struct string *st)
{
	CURL *curl;
	CURLcode res;

	curl_global_init(CURL_GLOBAL_DEFAULT);
	curl = curl_easy_init();

	if (!curl)
	{
		fprintf(stderr, "Failed to initialize CURL\n");
		curl_global_cleanup();
		return 1;
	}

	curl_easy_setopt(curl, CURLOPT_URL, url);

	if (cfg->verify_cert)
	{
		curl_set_ssl_verify(curl, cfg->cacert);
	}
	else
	{
		curl_set_ssl_noverify(curl);
	}
	curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, curl_recv);
	curl_easy_setopt(curl, CURLOPT_WRITEDATA, st);

	res = curl_easy_perform(curl);

	if (res != CURLE_OK)
	{
		fprintf(stderr, "curl_easy_perform() failed: %s\n",
						curl_easy_strerror(res));
		curl_easy_cleanup(curl);
		curl_global_cleanup();
		return 1;
	}

	curl_easy_cleanup(curl);
	curl_global_cleanup();

	return 0;
}
