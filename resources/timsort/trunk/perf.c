#include <stdlib.h>
#include <errno.h>
#include <stdio.h>
#include <sys/time.h>
#include <math.h>

#include "timsort.h"
#include "timsort1.h"

/*
 * -----------------------------------------------------------------------------
 *  Configure Structure
 * -----------------------------------------------------------------------------
 */
typedef struct perfContext
{
    int32_t      mDoVerify;
    char        *mFileName;

    int32_t      mCount;            /* read from input file */

    uint32_t    *mArrayToSort;      /* array to sort */

    void       (*mSortFunc)(void *, size_t, size_t, int (*compar)(const void *, const void *));
} perfContext;

static void perfContextInit(perfContext *aContext)
{
    aContext->mDoVerify      = -1;
    aContext->mFileName      = NULL;
    aContext->mCount         = -1;

    aContext->mArrayToSort   = NULL;
}

/*
 * -----------------------------------------------------------------------------
 *  Wrappers for heapsort and mergesort
 * -----------------------------------------------------------------------------
 */
static void mergesortLibc(void    *base,
                          size_t   nel,
                          size_t   width,
                          int    (*compar)(const void *, const void *))
{
    (void)mergesort(base, nel, width, compar);
}

static void heapsortLibc(void    *base,
                         size_t   nel,
                         size_t   width,
                         int    (*compar)(const void *, const void *))
{
    (void)heapsort(base, nel, width, compar);
}

/*
 * -----------------------------------------------------------------------------
 *  Verifying Sorted Array
 * -----------------------------------------------------------------------------
 */
static int32_t verifyArrayIsSorted(uint32_t *aArray, int32_t aCount)
{
    int32_t i;

#if 0
    (void)printf("\n");
    for (i = 0; i < aCount; i++)
    {
        (void)printf("%u\n", *(aArray + i));
    }
#endif

    for (i = 1; i < aCount; i++)
    {
        if (*(aArray + i - 1) > *(aArray + i)) return -1;
    }

    return 0;
}

/*
 * -----------------------------------------------------------------------------
 *  Compare function to deliver to sorting functions
 * -----------------------------------------------------------------------------
 */
static int32_t compareFunc(const void *aElem1, const void *aElem2)
{
    if (*(uint32_t *)aElem1 > *(uint32_t *)aElem2)
    {
        return 1;
    }
    else if (*(uint32_t *)aElem1 < *(uint32_t *)aElem2)
    {
        return -1;
    }
    else
    {
        return 0;
    }
}

/*
 * -----------------------------------------------------------------------------
 *  Allocating And Filling Array
 * -----------------------------------------------------------------------------
 */
static int32_t getCountFromFile(FILE *aFileHandle)
{
    int32_t  sCount = 0;
    char     sFirstLine[1024] = {0,};

    if (fgets(sFirstLine, sizeof(sFirstLine), aFileHandle) == NULL)
    {
        if (ferror(aFileHandle) != 0)
        {
            /* error */
            (void)fprintf(stderr, "error : reading file.\n");
            clearerr(aFileHandle);
            exit(1);
        }
        else
        {
            if (feof(aFileHandle) != 0)
            {
                /* eof */
                (void)fprintf(stderr, "%s\n", sFirstLine);
                (void)fprintf(stderr, "eof\n");
                exit(1);
            }
            else
            {
                abort();
            }
        }
    }

    if (sFirstLine[0] != '#')
    {
        (void)fprintf(stderr, "error : invalid file format.\n");
        exit(1);
    }
    else
    {
        char    *sEndPtr = NULL;

        sCount = strtol(sFirstLine + 1, &sEndPtr, 10);

        if (errno == ERANGE)
        {
            (void)fprintf(stderr, "error : the count is out of range.\n");
            exit(1);
        }
        else
        {
            if (*sEndPtr != '\n')
            {
                (void)fprintf(stderr, "error : invalid file format.\n");
                (void)fprintf(stderr, "endptr : %c\n", *sEndPtr);
                exit(1);
            }
            else
            {
            }
        }
    }

    return sCount;
}

static void createArray(perfContext *aContext)
{
    /*
     * Note : It is ubsurd to allocate a linear memory of such a big size.
     *        Operating system might swap out the memory if the system has not enough memory.
     *        and it might undermine the credibility of this performance test.
     */
    aContext->mArrayToSort = malloc(aContext->mCount * sizeof(uint32_t));

    if (aContext->mArrayToSort == NULL)
    {
        (void)fprintf(stderr, "error : malloc fail\n");
        exit(0);
    }
    else
    {
    }
}

static void destroyArray(uint32_t *sArray)
{
    free(sArray);
}

static void fillArray(FILE *aFileHandle, perfContext *aContext)
{
    int32_t  i;
    uint32_t sNumber;

    for (i = 0; i < aContext->mCount; i++)
    {
        if (fscanf(aFileHandle, "%u\n", &sNumber) == EOF)
        {
            (void)fprintf(stderr, "error : value count does not match\n");
            exit(1);
        }
        else
        {
        }

        aContext->mArrayToSort[i] = sNumber;
    }
}

static void createAndFillArray(perfContext *aContext)
{
    FILE *sFileHandle      = NULL;

    sFileHandle = fopen(aContext->mFileName, "r");

    if (sFileHandle == NULL)
    {
        if (errno == ENOENT)
        {
            (void)fprintf(stderr, "File not found.\n");
            exit(1);
        }
        else
        {
            (void)fprintf(stderr, "Unexpected error. %s (errno %d)\n", strerror(errno), errno);
            exit(1);
        }
    }
    else
    {
    }

    aContext->mCount = getCountFromFile(sFileHandle);

    createArray(aContext);

    fillArray(sFileHandle, aContext);
}

/*
 * -----------------------------------------------------------------------------
 *  Processing Command Line Arguments
 * -----------------------------------------------------------------------------
 */
static void printUsageAndExit(char *aProgramName)
{
    (void)fprintf(stderr, "Usage : %s [ -v ] <sorting_algorithm> <input_file_name>\n"
                          "    If -v is specified, the program verifies sorted array.\n"
                          "    Available sorting algorithms :\n"
                          "        quick\n"
                          "        merge\n"
                          "        heap\n"
                          "        tim (index)\n"
                          "        tim1 (pointer)\n",
                          aProgramName);
    exit(1);
}

static void processArgDetermineSortFunc(char        *aProgramName,
                                        char        *aAlgorithmName,
                                        perfContext *aContext)
{
    if (strcmp(aAlgorithmName, "quick") == 0)
    {
        aContext->mSortFunc = qsort;
    }
    else if (strcmp(aAlgorithmName, "merge") == 0)
    {
        aContext->mSortFunc = mergesortLibc;
    }
    else if (strcmp(aAlgorithmName, "heap") == 0)
    {
        aContext->mSortFunc = heapsortLibc;
    }
    else if (strcmp(aAlgorithmName, "tim") == 0)
    {
        aContext->mSortFunc = timsort;
    }
    else if (strcmp(aAlgorithmName, "tim1") == 0)
    {
        aContext->mSortFunc = timsort1;
    }
    else
    {
        printUsageAndExit(aProgramName);
    }
}

static void processArg(int32_t aArgc, char *aArgv[], perfContext *aContext)
{
    if (aArgc == 4)
    {
        /*
         * There is an option specified.
         */
        if (strcmp(aArgv[1], "-v") == 0)
        {
            aContext->mDoVerify = 1;
        }
        else
        {
            printUsageAndExit(aArgv[0]);
        }

        processArgDetermineSortFunc(aArgv[0], aArgv[2], aContext);
        aContext->mFileName = aArgv[3];
    }
    else if (aArgc == 3)
    {
        /*
         * No option specified
         */
        processArgDetermineSortFunc(aArgv[0], aArgv[1], aContext);
        aContext->mFileName = aArgv[2];
    }
    else
    {
        /*
         * Invalid number of arguments.
         */
        printUsageAndExit(aArgv[0]);
    }

    if (aContext->mDoVerify < 0) aContext->mDoVerify = 0; /* do not verify unless -v is provided */
}

/*
 * -----------------------------------------------------------------------------
 *  Main
 * -----------------------------------------------------------------------------
 */
int32_t main(int32_t aArgc, char *aArgv[])
{
    perfContext     sContext;

    struct timeval  sStart, sEnd;
    int32_t         sSeconds, sUseconds;

    uint32_t       *sArray;

    /*
     * Init
     */
    perfContextInit(&sContext);

    processArg(aArgc, aArgv, &sContext);

    /*
     * Allocate memory and load data
     */
    (void)fprintf(stderr, "Reading data...\n");
    createAndFillArray(&sContext);

    sArray = sContext.mArrayToSort;

    /*
     * Sort it!
     */
    (void)fprintf(stderr, "Start sorting...\n");
    (void)gettimeofday(&sStart, NULL);

    (*sContext.mSortFunc)(sArray, sContext.mCount, sizeof(uint32_t), compareFunc);

    (void)gettimeofday(&sEnd, NULL);
    (void)fprintf(stderr, "Completed sorting.\n");

    /*
     * Calculate time
     */
    sSeconds  = sEnd.tv_sec - sStart.tv_sec;
    sUseconds = sEnd.tv_usec - sStart.tv_usec;

    if (sUseconds < 0)
    {
        sSeconds--;
        sUseconds += 1000000;
    }

    (void)fprintf(stderr, "\nIt took %d.%06d seconds to sort %d elements.\n\n",
                  sSeconds, sUseconds, sContext.mCount);

    /*
     * Verify if option is set
     */
    if (sContext.mDoVerify == 1)
    {
        (void)fprintf(stderr, "Checking if resulting array is correctly sorted...... ");

        if (verifyArrayIsSorted(sArray, sContext.mCount) == 0)
        {
            (void)fprintf(stderr, "OK\n");
        }
        else
        {
            (void)fprintf(stderr, "FAIL\n");
        }
    }

    /*
     * Free memory
     */
    destroyArray(sArray);

    return 0;
}
