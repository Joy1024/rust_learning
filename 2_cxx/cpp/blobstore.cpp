// src/blobstore.cc

#include "include/blobstore.h"

BlobstoreClient::BlobstoreClient() {}
BlobstoreClient::~BlobstoreClient()
{
    std::cout << __func__ << std::endl;
}

std::unique_ptr<BlobstoreClient> new_blobstore_client()
{
    return std::unique_ptr<BlobstoreClient>(new BlobstoreClient());
}