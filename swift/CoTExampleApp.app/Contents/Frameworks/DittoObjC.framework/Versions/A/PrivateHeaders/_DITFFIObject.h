//
// Copyright © 2024 DittoLive Incorporated. All rights reserved.
//

#import <Foundation/Foundation.h>
#import <DittoObjC/dittoffi.h>

NS_ASSUME_NONNULL_BEGIN

@interface _DITFFIObject : NSObject

- (nullable void *)take;

- (instancetype)init NS_UNAVAILABLE;
- (instancetype)initWithPointer:(void *)pointer
                        release:(void (^)(void *pointer))release NS_DESIGNATED_INITIALIZER;

@end

NS_ASSUME_NONNULL_END
