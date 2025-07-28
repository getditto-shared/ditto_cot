//
//  Copyright © 2021 DittoLive Incorporated. All rights reserved.
//

#import <DittoObjC/DITBluetoothLEConfig.h>

#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

/**
 Configuration for Bluetooth Low Energy transport. Part of `DITPeerToPeer`.
 */
@interface DITMutableBluetoothLEConfig : DITBluetoothLEConfig

@property (nonatomic, readwrite, getter=isEnabled) BOOL enabled;

@end

NS_ASSUME_NONNULL_END
