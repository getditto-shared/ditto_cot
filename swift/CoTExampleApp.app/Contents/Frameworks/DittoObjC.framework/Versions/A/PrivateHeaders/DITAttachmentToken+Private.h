//
//  Copyright © 2020 DittoLive Incorporated. All rights reserved.
//

#import <DittoObjC/DITAttachmentToken.h>

NS_ASSUME_NONNULL_BEGIN

@interface DITAttachmentToken ()

@property (nonatomic, readonly) NSData *id;
@property (nonatomic, readonly) NSUInteger len;
@property (nonatomic, readonly) NSDictionary<NSString *, NSString *> *metadata;

- (instancetype)new NS_UNAVAILABLE;
- (instancetype)init NS_UNAVAILABLE;
- (nullable instancetype)initWithInternalDictionaryRepresentation:
    (NSDictionary<NSString *, id> *)dictionaryRepresentation NS_DESIGNATED_INITIALIZER;
- (nullable instancetype)initWithPublicDictionaryRepresentation:
    (NSDictionary<NSString *, id> *)dictionaryRepresentation NS_DESIGNATED_INITIALIZER;

@end

NS_ASSUME_NONNULL_END
