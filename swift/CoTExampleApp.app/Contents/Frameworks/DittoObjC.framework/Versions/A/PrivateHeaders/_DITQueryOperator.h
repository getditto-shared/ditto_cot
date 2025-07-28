//
//  Copyright © 2020 DittoLive Incorporated. All rights reserved.
//

#import <Foundation/Foundation.h>

#import <DittoObjC/dittoffi.h>

struct Ditto_WriteTransaction;
struct COrderByParam;

@class _DITDittoHandleWrapper;
@class _DITDocumentHandleWrapper;
@class _DITOrderBy;
@class DITDocument;
@class DITDocumentID;
@class DITMutableDocument;
@class DITUpdateResult;

NS_ASSUME_NONNULL_BEGIN

@interface _DITQueryOperator : NSObject

+ (NSArray<DITDocument *> *)execUsingQuery:(NSString *)query
                             queryArgsData:(nullable NSData *)queryArgsData
                            collectionName:(NSString *)collName
                                   orderBy:(_DITOrderBy *)orderBy
                                     limit:(int)limit
                                    offset:(uint)offset
                        dittoHandleWrapper:(_DITDittoHandleWrapper *)dittoHandleWrapper
                                  writeTxn:(nullable CWriteTransaction_t *)writeTxn;

+ (NSArray<_DITDocumentHandleWrapper *> *)
    execWithDocumentHandlesUsingQuery:(NSString *)query
                        queryArgsData:(nullable NSData *)queryArgsData
                       collectionName:(NSString *)collName
                              orderBy:(_DITOrderBy *)orderBy
                                limit:(int)limit
                               offset:(uint)offset
                   dittoHandleWrapper:(_DITDittoHandleWrapper *)dittoHandleWrapper
                             writeTxn:(nullable CWriteTransaction_t *)writeTxn;

+ (NSArray<DITDocumentID *> *)evictUsingQuery:(NSString *)query
                                queryArgsData:(nullable NSData *)queryArgsData
                               collectionName:(NSString *)collName
                                      orderBy:(_DITOrderBy *)orderBy
                                        limit:(int)limit
                                       offset:(uint)offset
                           dittoHandleWrapper:(_DITDittoHandleWrapper *)dittoHandleWrapper
                                     writeTxn:(CWriteTransaction_t *)writeTxn;

+ (NSArray<DITDocumentID *> *)removeUsingQuery:(NSString *)query
                                 queryArgsData:(nullable NSData *)queryArgsData
                                collectionName:(NSString *)collName
                                       orderBy:(_DITOrderBy *)orderBy
                                         limit:(int)limit
                                        offset:(uint)offset
                            dittoHandleWrapper:(_DITDittoHandleWrapper *)dittoHandleWrapper
                                      writeTxn:(CWriteTransaction_t *)writeTxn;

+ (NSDictionary<DITDocumentID *, NSArray<DITUpdateResult *> *> *)
      updateUsingQuery:(NSString *)query
         queryArgsData:(nullable NSData *)queryArgsData
        collectionName:(NSString *)collName
               orderBy:(_DITOrderBy *)orderBy
                 limit:(int)limit
                offset:(uint)offset
    dittoHandleWrapper:(_DITDittoHandleWrapper *)dittoHandleWrapper
              writeTxn:(CWriteTransaction_t *)writeTxn
           updateBlock:(void (^)(NSArray<DITMutableDocument *> *))block;

+ (BOOL)experimentalAddDQLSubscriptionUsingQuery:(NSString *)query
                                   queryArgsData:(nullable NSData *)queryArgsData
                              dittoHandleWrapper:(_DITDittoHandleWrapper *)dittoHandleWrapper
                                        ffiError:(dittoffi_error_t *_Nullable *_Nonnull)ffiError
                                           error:(NSError *_Nullable __autoreleasing *)error;

+ (BOOL)experimentalRemoveDQLSubscriptionUsingQuery:(NSString *)query
                                      queryArgsData:(nullable NSData *)queryArgsData
                                 dittoHandleWrapper:(_DITDittoHandleWrapper *)dittoHandleWrapper
                                           ffiError:(dittoffi_error_t *_Nullable *_Nonnull)ffiError
                                              error:(NSError *_Nullable __autoreleasing *)error;

@end

NS_ASSUME_NONNULL_END
