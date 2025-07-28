//
//  Copyright © 2020 DittoLive Incorporated. All rights reserved.
//

#import <DittoObjC/DITAttachmentFetcher.h>

@class DITAttachmentFetchEvent;
@class DITAttachmentToken;
@class _DITDittoHandleWrapper;

NS_ASSUME_NONNULL_BEGIN

@interface DITAttachmentFetcher ()

@property (nonatomic, readonly) _DITDittoHandleWrapper *dittoHandleWrapper;
@property (nonatomic, readonly) dispatch_queue_t deliveryDispatchQueue;
@property (nonatomic, readonly) void (^onFetchEvent)(DITAttachmentFetchEvent *);
@property (nonatomic, readonly) DITAttachmentToken *token;
@property (nonatomic) int64_t cancelToken;

- (nullable instancetype)initWithAttachmentToken:(DITAttachmentToken *)attachmentToken
                                   deliveryQueue:(dispatch_queue_t)dispatchQueue
                                    onFetchEvent:(void (^)(DITAttachmentFetchEvent *))onFetchEvent
                              dittoHandleWrapper:(_DITDittoHandleWrapper *)dittoHandleWrapper
                                           error:(NSError *_Nullable *)error;
@end

NS_ASSUME_NONNULL_END
