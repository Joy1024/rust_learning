// include/blobstore.h

#pragma once
#include <memory>
#include <string>
#include <iostream>

class BlobstoreClient {
public:
  BlobstoreClient();

  ~BlobstoreClient();

  const std::string& get(const std::string& str) const {
    std::cout << "get:" << str << std::endl;
    return str;
  }
};

std::unique_ptr<BlobstoreClient> new_blobstore_client();