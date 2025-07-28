//
//  Copyright © 2019 DittoLive Incorporated. All rights reserved.
//

#import <DittoObjCUtil/dittoffi.h>
#import <Foundation/Foundation.h>

NS_ASSUME_NONNULL_BEGIN

#define DITLogError(args...) DITLog(C_LOG_LEVEL_ERROR, args);
#define DITLogWarning(args...) DITLog(C_LOG_LEVEL_WARNING, args);
#define DITLogInfo(args...) DITLog(C_LOG_LEVEL_INFO, args);
#define DITLogDebug(args...) DITLog(C_LOG_LEVEL_DEBUG, args);
#define DITLogVerbose(args...) DITLog(C_LOG_LEVEL_VERBOSE, args);

void DITLog(CLogLevel_t logLevel, NSString *format, ...) NS_FORMAT_FUNCTION(2,3);

NS_ASSUME_NONNULL_END
