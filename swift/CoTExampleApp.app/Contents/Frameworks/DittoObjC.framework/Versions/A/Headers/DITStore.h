//
//  Copyright © 2020 DittoLive Incorporated. All rights reserved.
//

#import <Foundation/Foundation.h>

@class DITAttachment;
@class DITAttachmentFetchEvent;
@class DITAttachmentFetcher;
@class DITAttachmentToken;
@class DITCollection;
@class DITLiveQuery;
@class DITPendingCollectionsOperation;
@class DITWriteTransaction;
@class DITWriteTransactionResult;

NS_ASSUME_NONNULL_BEGIN

/**
 The entrypoint for all actions that relate to data stored by Ditto.

 `DITStore` provides access to `DITCollection`s, a write transaction API, and a query hash API.
 */
@interface DITStore : NSObject

/**
 Returns a `DITCollection` with the provided name.

 @param collectionName The name of the collection.

 @return A `DITCollection`.
 */
- (DITCollection *)objectForKeyedSubscript:(NSString *)collectionName;

/**
 Returns a `DITCollection` with the provided name.

 @param name The name of the collection.
A collection name is valid if:
* its length is less than 100
* it is not empty
* it does not contain the char '\0'
* it does not begin with "$TS_"
 @return A `DITCollection`.
 */
- (DITCollection *)collection:(NSString *)name;

/**
 Returns a list of the names of collections in the store.

 @return A list of collection names found in the store.
 */
- (NSArray<NSString *> *)collectionNames;

/**
 Returns an object that lets you fetch or observe the collections in the store.

 @return An object that lets you fetch or observe the collections in the store.
 */
- (DITPendingCollectionsOperation *)collections;

/**
 Returns a hash representing the current version of the given queries.

 When a document matching such queries gets mutated, the hash will change as well.

 Please note that the hash depends on how queries are constructed, so you should make sure to always
 compare hashes generated with the same set of queries.

 @param liveQueries A list of `DITLiveQuery` objects that you want to get the hash for.

 @return A hash representing the current version of the given queries.
 */
- (NSUInteger)queriesHash:(NSArray<DITLiveQuery *> *)liveQueries;

/**
 Returns a sequence of English words representing the current version of the given queries.

 When a document matching such queries gets mutated, the words will change as well.

 Please note that the resulting sequence of words depends on how queries are constructed, so you
 should make sure to always compare hashes generated with the same set of queries.

 @param liveQueries A list of `DITLiveQuery` objects that you want to get the mnemonic hash for.

 @return A string representing the current version of the given queries.
 */
- (NSString *)queriesHashMnemonic:(NSArray<DITLiveQuery *> *)liveQueries;

/**
 Allows you to group multiple operations together that affect multiple documents, potentially across
 multiple collections.

 @param block A block that provides access to a write transaction object that can be used to perform
 operations on the store.

 @return A list of `DITWriteTransactionResult`s. There is a result for each operation performed as
 part of the write transaction.
 */
- (NSArray<DITWriteTransactionResult *> *)write:(void (^)(DITWriteTransaction *))block;

/**
 Allows you to group multiple operations together that affect multiple documents, potentially across
 multiple collections.

 @param logHint A hint about the transaction, to improve logs or diagnostics.

 @param block A block that provides access to a write transaction object that can be used to perform
 operations on the store.

 @return A list of `DITWriteTransactionResult`s. There is a result for each operation performed as
 part of the write transaction.
 */
- (NSArray<DITWriteTransactionResult *> *)writeWithLogHint:(NSString *)logHint
                                                     block:(void (^)(DITWriteTransaction *))block
    NS_SWIFT_NAME(write(logHint:block:));

// No docs because DittoObjC is deprecated. Temporary to aid the Swift implementation.
- (nullable DITAttachment *)newAttachment:(NSString *)path
                                 metadata:(nullable NSDictionary<NSString *, NSString *> *)metadata
                                    error:(NSError *_Nullable *)error;

// No docs because DittoObjC is deprecated. Temporary to aid the Swift implementation.
- (nullable DITAttachmentFetcher *)fetchAttachment:(DITAttachmentToken *)token
                                      onFetchEvent:(void (^)(DITAttachmentFetchEvent *))onFetchEvent
                                             error:(NSError *_Nullable *)error;

// No docs because DittoObjC is deprecated. Temporary to aid the Swift implementation.
- (nullable DITAttachmentFetcher *)fetchAttachment:(DITAttachmentToken *)token
                                     deliveryQueue:(dispatch_queue_t)dispatchQueue
                                      onFetchEvent:(void (^)(DITAttachmentFetchEvent *))onFetchEvent
                                             error:(NSError *_Nullable *)error;

@end

NS_ASSUME_NONNULL_END
