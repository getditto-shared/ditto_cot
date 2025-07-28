//
// Copyright © 2021 DittoLive Incorporated. All rights reserved.
//

#import <Foundation/Foundation.h>

#import <DittoObjC/DITConnectionPriority.h>

NS_ASSUME_NONNULL_BEGIN

@class DITDitto;
@class DITIdentity;

/**
 Upcoming SDK features made available for prototyping.

 @warning Experimental functionality should not be used in production
 applications as it may be changed or removed at any time, and may not
 have the same security features.
 */
@interface DITExperimental : NSObject

- (instancetype)init NS_UNAVAILABLE;

/**
 Experimental factory method creating a `DITDitto` object with on-disk
 encryption enabled if `passphrase` is given. Otherwise data will be
 written to disk unencrypted, just as with regular `DITDitto` initializers.

 @param identity Provide the identity of the entity that is interacting with Ditto.
 @param historyTrackingEnabled Whether or not you want history tracking enabled.
 @param directory The directory that will be used to persist Ditto data.
 @param passphrase If given, the data will be encrypted on disk with the passphrase.
 @param error If an error occurs, upon return contains an `NSError` object that
 describes the problem. If you are not interested in possible errors, pass in `NULL`.
 */
+ (nullable DITDitto *)openWithIdentity:(DITIdentity *)identity
                 historyTrackingEnabled:(BOOL)historyTrackingEnabled
                   persistenceDirectory:(nullable NSURL *)directory
                             passphrase:(nullable NSString *)passphrase
                                  error:(NSError *_Nullable *)error;

/** Transcodes CBOR to a JSON string. */
+ (nullable NSData *)JSONDataByTranscodingCBORData:(NSData *)cbor
                                             error:(NSError *_Nullable __autoreleasing *)error;

@end

NS_ASSUME_NONNULL_END
