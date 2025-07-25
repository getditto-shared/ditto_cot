import SwiftUI
#if os(macOS)
import AppKit
#endif

/// SwiftUI view for CoT chat messaging
@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
public struct CoTChatView: View {
    @StateObject private var viewModel: CoTEventViewModel
    @State private var messageText = ""
    @State private var currentCallsign = "USER-1"
    @State private var selectedRoom = "All Chat Rooms"
    @FocusState private var isMessageFieldFocused: Bool
    
    public init(observable: CoTObservable) {
        self._viewModel = StateObject(wrappedValue: CoTEventViewModel(observable: observable))
    }
    
    public var body: some View {
        VStack {
            // Chat header
            ChatHeader(
                currentCallsign: $currentCallsign,
                selectedRoom: $selectedRoom,
                isConnected: viewModel.isConnected
            )
            
            // Messages list
            ScrollViewReader { proxy in
                ScrollView {
                    LazyVStack(spacing: 8) {
                        ForEach(viewModel.recentChatMessages) { message in
                            ChatMessageBubble(
                                message: message,
                                isFromCurrentUser: message.from == currentCallsign
                            )
                            .id(message.id)
                        }
                    }
                    .padding(.horizontal)
                }
                .onChange(of: viewModel.recentChatMessages.count) { _ in
                    // Auto-scroll to bottom when new messages arrive
                    if let lastMessage = viewModel.recentChatMessages.last {
                        withAnimation(.easeOut(duration: 0.3)) {
                            proxy.scrollTo(lastMessage.id, anchor: .bottom)
                        }
                    }
                }
            }
            
            // Message input
            MessageInputView(
                messageText: $messageText,
                isMessageFieldFocused: $isMessageFieldFocused,
                onSend: sendMessage
            )
        }
        .navigationTitle("CoT Chat")
        #if os(iOS)
        .navigationBarTitleDisplayMode(.inline)
        #endif
        .onAppear {
            viewModel.refreshEvents()
        }
    }
    
    private func sendMessage() {
        guard !messageText.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty else { return }
        
        Task {
            do {
                try await viewModel.sendChatMessage(
                    message: messageText,
                    room: selectedRoom,
                    callsign: currentCallsign
                )
                await MainActor.run {
                    messageText = ""
                }
            } catch {
                // Handle error (could show alert)
                print("Failed to send message: \(error)")
            }
        }
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct ChatHeader: View {
    @Binding var currentCallsign: String
    @Binding var selectedRoom: String
    let isConnected: Bool
    @State private var showingSettings = false
    
    var body: some View {
        VStack(spacing: 8) {
            HStack {
                VStack(alignment: .leading, spacing: 2) {
                    Text("Room: \(selectedRoom)")
                        .font(.headline)
                    Text("Callsign: \(currentCallsign)")
                        .font(.caption)
                        .foregroundColor(.secondary)
                }
                
                Spacer()
                
                ConnectionStatusView(isConnected: isConnected)
                
                Button("Settings") {
                    showingSettings = true
                }
                .font(.caption)
            }
            .padding(.horizontal)
            
            Divider()
        }
        #if os(iOS)
        .background(Color(.systemGroupedBackground))
        #else
        .background(Color(NSColor.controlBackgroundColor))
        #endif
        .sheet(isPresented: $showingSettings) {
            ChatSettingsView(
                currentCallsign: $currentCallsign,
                selectedRoom: $selectedRoom
            )
        }
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct ChatMessageBubble: View {
    let message: ChatMessageModel
    let isFromCurrentUser: Bool
    
    var body: some View {
        HStack {
            if isFromCurrentUser {
                Spacer()
            }
            
            VStack(alignment: isFromCurrentUser ? .trailing : .leading, spacing: 4) {
                if !isFromCurrentUser {
                    Text(message.from)
                        .font(.caption)
                        .fontWeight(.semibold)
                        .foregroundColor(.secondary)
                }
                
                Text(message.message)
                    .padding(.horizontal, 12)
                    .padding(.vertical, 8)
                    .background(
                        isFromCurrentUser ? Color.blue : backgroundColorForMessage
                    )
                    .foregroundColor(
                        isFromCurrentUser ? .white : .primary
                    )
                    .cornerRadius(16)
                
                Text(message.timestamp, style: .time)
                    .font(.caption2)
                    .foregroundColor(.secondary)
            }
            .frame(maxWidth: 250, alignment: isFromCurrentUser ? .trailing : .leading)
            
            if !isFromCurrentUser {
                Spacer()
            }
        }
    }
    
    private var backgroundColorForMessage: Color {
        #if os(iOS)
        return Color(.systemGray5)
        #else
        return Color(NSColor.controlBackgroundColor)
        #endif
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct MessageInputView: View {
    @Binding var messageText: String
    var isMessageFieldFocused: FocusState<Bool>.Binding
    let onSend: () -> Void
    
    var body: some View {
        HStack(spacing: 8) {
            #if os(macOS)
            if #available(macOS 13.0, *) {
                TextField("Type a message...", text: $messageText, axis: .vertical)
                    .textFieldStyle(RoundedBorderTextFieldStyle())
                    .focused(isMessageFieldFocused)
                    .lineLimit(1...4)
                    .onSubmit {
                        onSend()
                    }
            } else {
                TextField("Type a message...", text: $messageText)
                    .textFieldStyle(RoundedBorderTextFieldStyle())
                    .focused(isMessageFieldFocused)
                    .onSubmit {
                        onSend()
                    }
            }
            #else
            TextField("Type a message...", text: $messageText, axis: .vertical)
                .textFieldStyle(RoundedBorderTextFieldStyle())
                .focused(isMessageFieldFocused)
                .lineLimit(1...4)
                .onSubmit {
                    onSend()
                }
            #endif
            
            Button(action: onSend) {
                Image(systemName: "arrow.up.circle.fill")
                    .font(.title2)
                    .foregroundColor(messageText.isEmpty ? .secondary : .blue)
            }
            .disabled(messageText.trimmingCharacters(in: .whitespacesAndNewlines).isEmpty)
        }
        .padding(.horizontal)
        .padding(.bottom, 8)
        #if os(iOS)
        .background(Color(.systemBackground))
        #else
        .background(Color(NSColor.windowBackgroundColor))
        #endif
    }
}

@available(iOS 15.0, macOS 12.0, watchOS 8.0, tvOS 15.0, *)
struct ChatSettingsView: View {
    @Binding var currentCallsign: String
    @Binding var selectedRoom: String
    @Environment(\.dismiss) private var dismiss
    
    private let commonRooms = [
        "All Chat Rooms",
        "Operations",
        "Command",
        "Logistics",
        "Medical",
        "Emergency"
    ]
    
    var body: some View {
        NavigationView {
            Form {
                Section("User Settings") {
                    HStack {
                        Text("Callsign")
                        Spacer()
                        TextField("Enter callsign", text: $currentCallsign)
                            .textFieldStyle(RoundedBorderTextFieldStyle())
                            #if os(iOS)
                            .autocapitalization(.allCharacters)
                            #endif
                            .frame(maxWidth: 120)
                    }
                }
                
                Section("Chat Room") {
                    Picker("Room", selection: $selectedRoom) {
                        ForEach(commonRooms, id: \.self) { room in
                            Text(room).tag(room)
                        }
                    }
                    .pickerStyle(.menu)
                }
                
                Section("Instructions") {
                    VStack(alignment: .leading, spacing: 8) {
                        Text("• Set your callsign to identify yourself in chat")
                        Text("• Select a room to join specific conversations")
                        Text("• Messages are synchronized across all connected devices")
                        Text("• Chat messages follow CoT protocol standards")
                    }
                    .font(.caption)
                    .foregroundColor(.secondary)
                }
            }
            .navigationTitle("Chat Settings")
            #if os(iOS)
            .navigationBarTitleDisplayMode(.inline)
            #endif
            .toolbar {
                #if os(iOS)
                ToolbarItem(placement: .navigationBarTrailing) {
                    Button("Done") {
                        dismiss()
                    }
                }
                #else
                ToolbarItem(placement: .confirmationAction) {
                    Button("Done") {
                        dismiss()
                    }
                }
                #endif
            }
        }
    }
}