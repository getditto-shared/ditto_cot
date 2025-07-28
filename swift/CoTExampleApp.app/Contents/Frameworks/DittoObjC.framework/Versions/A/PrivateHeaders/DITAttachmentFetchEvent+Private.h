//
//  Copyright © 2020 DittoLive Incorporated. All rights reserved.
//

#import <DittoObjC/DITAttachmentFetchEvent.h>

@class DITAttachment;
@class DITAttachmentFetcher;

NS_ASSUME_NONNULL_BEGIN

@interface DITAttachmentFetchEvent ()

@property (weak, nonatomic, nullable, readonly) DITAttachmentFetcher *attachmentFetcher;

- (instancetype)initWithAttachmentFetcher:(DITAttachmentFetcher *)attachmentFetcher
    NS_DESIGNATED_INITIALIZER;

@end

@interface DITAttachmentFetchEventCompleted ()
- (instancetype)initWithAttachment:(DITAttachment *)attachment
                 attachmentFetcher:(DITAttachmentFetcher *)attachmentFetcher;
@end

@interface DITAttachmentFetchEventProgress ()
- (instancetype)initWithDownloadedBytes:(NSUInteger)downloadedBytes
                             totalBytes:(NSUInteger)totalBytes
                      attachmentFetcher:(DITAttachmentFetcher *)attachmentFetcher;
@end

@interface DITAttachmentFetchEventDeleted ()
- (instancetype)initWithAttachmentFetcher:(DITAttachmentFetcher *)attachmentFetcher;
@end

NS_ASSUME_NONNULL_END
