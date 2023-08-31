#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <errno.h>
#include <math.h>
#include <time.h>
#include <stdint.h>
#include <assert.h>

/*
 * -----------------------------------------------------------------------------
 *  Configure
 * -----------------------------------------------------------------------------
 */
static const char *gPatternName[] = 
{
    "None",
    "random",
    "sorted",
    "reversed",
    "sinwave1",
    "chainsaw",
    "identical",
    NULL
};

typedef enum
{
    GEN_DATA_ORDER_ASCENDING,
    GEN_DATA_ORDER_DESCENDING
} genDataOrder;

typedef enum
{
    GEN_DATA_PATTERN_NONE,
    GEN_DATA_PATTERN_RANDOM,
    GEN_DATA_PATTERN_SORTED,
    GEN_DATA_PATTERN_SORTED_REVERSE,
    GEN_DATA_PATTERN_SINWAVE1,
    GEN_DATA_PATTERN_CHAINSAW,
    GEN_DATA_PATTERN_IDENTICAL,
    /* GEN_DATA_PATTERN_ALMOST, */
    GEN_DATA_PATTERN_MAX,
} genDataPattern;

typedef struct genDataConf
{
    int32_t         mCount;
    genDataPattern  mPattern;
} genDataConf;

static void genDataConfInit(genDataConf *aConf)
{
    aConf->mCount   = -1;
    aConf->mPattern = GEN_DATA_PATTERN_NONE;
}

/*
 * -----------------------------------------------------------------------------
 *  Already Sorted Array
 * -----------------------------------------------------------------------------
 */
static int genCmpInt(const void *a, const void *b)
{
    return (*(uint32_t *)a - *(uint32_t *)b);
}

static void genDataMakeSortedArray(uint32_t *aArray, uint32_t aSize)
{
    uint32_t i;
    char     sRandState[256];

    (void)initstate(time(NULL), sRandState, 256);

    for (i = 0; i < aSize; i++)
    {
        aArray[i] = (uint32_t)random();
    }

    qsort(aArray, aSize, sizeof(uint32_t), genCmpInt);
}

static void genDataGenerateSorted(uint32_t aSampleCnt, genDataOrder aOrder)
{
    uint32_t  i;
    uint32_t  sStart = 0;
    int32_t   sDelta = 0;
    uint32_t  sIndex;

    uint32_t *sArray = NULL;

    sArray = malloc(sizeof(uint32_t) * aSampleCnt);
    assert(sArray != NULL);

    genDataMakeSortedArray(sArray, aSampleCnt);

    switch (aOrder)
    {
        case GEN_DATA_ORDER_ASCENDING:
            sStart = 0;
            sDelta = 1;
            break;
        case GEN_DATA_ORDER_DESCENDING:
            sStart = aSampleCnt - 1;
            sDelta = -1;
            break;
    }

    sIndex = sStart;

    for (i = 0; i < aSampleCnt; i++)
    {
        (void)fprintf(stdout, "%u\n", sArray[sIndex]);
        sIndex += sDelta;
    }

    free(sArray);
}

/*
 * -----------------------------------------------------------------------------
 *  Almost Sorted Array with 0.1 * N permutations
 * -----------------------------------------------------------------------------
 */
#if 0
static void genDataGenerateAlmostSorted(uint32_t aSampleCnt)
{
    uint32_t *sArray = NULL;

    sArray = malloc(sizeof(uint32_t) * aSampleCnt);
    assert(sArray != NULL);

    genDataMakeSortedArray(sArray, aSampleCnt);

    free(sArray);
}
#endif

/*
 * -----------------------------------------------------------------------------
 *  Sine wave type 1
 * -----------------------------------------------------------------------------
 */
static void genDataGenerateSinWave1(uint32_t aSampleCnt, uint32_t aCycleCount)
{
    uint32_t  i;

    double    x = 0;
    double    sValue;
    double    sDelta;

    sDelta = 2 * aCycleCount * M_PI / aSampleCnt;

    for (i = 0; i < aSampleCnt; i++)
    {
        x += sDelta;

        sValue  = (sin(x) + 1) * 100000 * x;

        (void)fprintf(stdout, "%u\n", 0xFFFFFFFF - (uint32_t)sValue);
        // sArray[i] = (uint32_t)sValue;
    }
}

/*
 * -----------------------------------------------------------------------------
 *  Sine wave type 2
 * -----------------------------------------------------------------------------
 */
#if 0
static void genDataGenerateSinWave2(uint32_t aSampleCnt, uint32_t aCycleCount)
{
    uint32_t  i;

    double    x = 0;
    double    sValue;
    double    sDelta;

    sDelta = 2 * aCycleCount * M_PI / aSampleCnt;

    for (i = 0; i < aSampleCnt; i++)
    {
        x += sDelta;

        sValue  = (sin(x) + 1) * 10000000;
        sValue += (cos(M_PI * x) + 1) * 10000000;

        (void)fprintf(stdout, "%u\n", (uint32_t)sValue);
    }
}
#endif

/*
 * -----------------------------------------------------------------------------
 *  Chainsaw (monotonic sinwave)
 * -----------------------------------------------------------------------------
 */
static void genDataGenerateChainsaw(uint32_t aSampleCnt, uint32_t aCycleCount)
{
    uint32_t  i;

    double    x = 0;
    double    sValue;
    double    sDelta;

    sDelta = 2 * aCycleCount * M_PI / aSampleCnt;

    for (i = 0; i < aSampleCnt; i++)
    {
        x += sDelta;

        sValue  = (sin(x) + 1) * 100000;

        (void)fprintf(stdout, "%u\n", 0xFFFFFFFF - (uint32_t)sValue);
    }
}

/*
 * -----------------------------------------------------------------------------
 *  Identical Values
 * -----------------------------------------------------------------------------
 */
static void genDataGenerateIdentical(uint32_t aSampleCnt)
{
    uint32_t  i;

    for (i = 0; i < aSampleCnt; i++)
    {
        (void)fprintf(stdout, "1234567\n");
    }
}

/*
 * -----------------------------------------------------------------------------
 *  Random
 * -----------------------------------------------------------------------------
 */
static void genDataGenerateRandom(uint32_t aSampleCnt)
{
    uint32_t  i;
    char      sRandState[256];

    (void)initstate(time(NULL), sRandState, 256);

    for (i = 0; i < aSampleCnt; i++)
    {
        (void)fprintf(stdout, "%u\n", (uint32_t)random());
    }
}

/*
 * -----------------------------------------------------------------------------
 *  Error handling routine
 * -----------------------------------------------------------------------------
 */
static void genDataPrintUsageAndExit(char *aProgramName)
{
    uint32_t i = 1;

    (void)fprintf(stderr, "Usage : %s [ options ]\n"
                          "  -c NUM      element count\n"
                          "  -p PATETERN\n", aProgramName);

    while (gPatternName[i] != NULL)
    {
        (void)fprintf(stderr, "        %s\n", gPatternName[i]);
        i++;
    }

    exit(1);
}

/*
 * -----------------------------------------------------------------------------
 *  Process command line arguments
 *      -c num : count
 *      -p pattern
 * -----------------------------------------------------------------------------
 */
static void processArg(int32_t aArgc, char *aArgv[], genDataConf *aConf)
{
    int32_t i;

    if (aArgc == 1)
    {
        genDataPrintUsageAndExit(aArgv[0]);
    }

    for (i = 1; i < aArgc; i++)
    {
        if (strcmp(aArgv[i], "-c") == 0)
        {
            /* -c : the number of samples */
            if (aConf->mCount < 0)
            {
                if (i + 1 < aArgc)
                {
                    int32_t  sCount;
                    char    *sEndPtr = NULL;
                    sCount = strtol(aArgv[i + 1], &sEndPtr, 10);
                    if (errno == ERANGE)
                    {
                        (void)fprintf(stderr, "error : the value provided with "
                                              "'%s' is out of range.\n", aArgv[i]);
                    }
                    else
                    {
                        if (*sEndPtr != '\0')
                        {
                            (void)fprintf(stderr, "error : option '%s' only accepts integer.\n", aArgv[i]);
                            exit(1);
                        }
                    }

                    aConf->mCount = sCount;
                    i++;
                }
                else
                {
                    (void)fprintf(stderr, "error : option '%s' needs to be "
                                          "provided with a value.\n", aArgv[i]);
                    genDataPrintUsageAndExit(aArgv[0]);
                }
            }
            else
            {
                (void)fprintf(stderr, "error : an option cannot be specified more than once.\n");
                exit(1);
            }
        }
        else if (strcmp(aArgv[i], "-p") == 0)
        {
            if (aConf->mPattern == GEN_DATA_PATTERN_NONE)
            {
                if (i + 1 < aArgc)
                {
                    uint32_t j;

                    i++;

                    for (j = 1; j < GEN_DATA_PATTERN_MAX; j++)
                    {
                        if (strcmp(aArgv[i], gPatternName[j]) == 0)
                        {
                            aConf->mPattern = j;
                            break;
                        }
                        else
                        {
                        }
                    }
                }
                else
                {
                    (void)fprintf(stderr, 
                                  "error : option '%s' needs to be "
                                  "provided with a source data pattern.\n", aArgv[i]);
                    genDataPrintUsageAndExit(aArgv[0]);
                }
            }
            else
            {
                (void)fprintf(stderr, "error : an option cannot be specified more than once.\n");
                exit(1);
            }
        }
        else
        {
            genDataPrintUsageAndExit(aArgv[0]);
        }
    }

    if (aConf->mCount < 0)
    {
        (void)fprintf(stderr, "error : sample count should be provided.\n");
        genDataPrintUsageAndExit(aArgv[0]);
    }
    else
    {
    }

    if (aConf->mPattern == GEN_DATA_PATTERN_NONE)
    {
        (void)fprintf(stderr, "error : data pattern must be provided.\n");
        genDataPrintUsageAndExit(aArgv[0]);
    }
    else
    {
    }
}

int32_t main(int32_t aArgc, char *aArgv[])
{
    genDataConf sConf;

    genDataConfInit(&sConf);
    processArg(aArgc, aArgv, &sConf);

    /*
     * Printing count
     */
    (void)fprintf(stdout, "#%d\n", sConf.mCount);

    /*
     * Printing data
     */
    switch (sConf.mPattern)
    {
        case GEN_DATA_PATTERN_RANDOM:
            genDataGenerateRandom(sConf.mCount);
            break;

        case GEN_DATA_PATTERN_SORTED:
            genDataGenerateSorted(sConf.mCount, GEN_DATA_ORDER_ASCENDING);
            break;

        case GEN_DATA_PATTERN_SORTED_REVERSE:
            genDataGenerateSorted(sConf.mCount, GEN_DATA_ORDER_DESCENDING);
            break;

        case GEN_DATA_PATTERN_SINWAVE1:
            genDataGenerateSinWave1(sConf.mCount, 10);
            break;

        case GEN_DATA_PATTERN_CHAINSAW:
            genDataGenerateChainsaw(sConf.mCount, 10);
            break;

        case GEN_DATA_PATTERN_IDENTICAL:
            genDataGenerateIdentical(sConf.mCount);
            break;

        case GEN_DATA_PATTERN_NONE:
        case GEN_DATA_PATTERN_MAX:
            abort();
            break;
    }

    return 0;
}

