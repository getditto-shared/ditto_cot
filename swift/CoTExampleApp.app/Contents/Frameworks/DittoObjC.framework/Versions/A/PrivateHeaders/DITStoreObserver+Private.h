//
//  Copyright © 2020 DittoLive Incorporated. All rights reserved.
//

#import <DittoObjC/DITStoreObserver.h>
#import <DittoObjC/dittoffi.h>

@class DITDocument;
@class DITSubscription;

@class _DITDittoHandleWrapper;
@class _DITDocumentHandleWrapper;
@class _DITOrderBy;

NS_ASSUME_NONNULL_BEGIN

@interface DITStoreObserver ()

@property (nonatomic, readonly) _DITDittoHandleWrapper *dittoHandleWrapper;
@property (nonatomic, readonly, nullable) DITSubscription *subscription;
@property (nonatomic, readonly) int64_t lqID;
@property (nonatomic, readonly) dispatch_queue_t deliveryDispatchQueue;
@property (nonatomic, readonly, nullable) void (^eventHandler)
    (dittoffi_query_result_t *, void (^_Nullable)(void));
@property (nonatomic, readonly, nullable) void (^signalNextBlock)(void);

+ (nullable instancetype)storeObserverWithQuery:(NSString *)query
                                  queryArgsData:(nullable NSData *)queryArgsData
                                   availability:(LiveQueryAvailability_t)availability
                             dittoHandleWrapper:(_DITDittoHandleWrapper *)dittoHandleWrapper
                          deliveryDispatchQueue:(dispatch_queue_t)dispatchQueue
                                       ffiError:(dittoffi_error_t *_Nullable *_Nonnull)ffiError
                                   eventHandler:(void (^)(dittoffi_query_result_t *,
                                                          void (^_Nullable)(void)))eventHandler
                                          error:(NSError *_Nullable *)error;

+ (NSArray<DITDocument *> *)documentsFrom:(Vec_CDocument_ptr_t)docsVec;
+ (NSArray<NSNumber *> *)indexesFrom:(slice_boxed_size_t)idxs_boxed;

@end

NS_ASSUME_NONNULL_END
